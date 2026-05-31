use futures::StreamExt;
use reqwest::header::ACCEPT;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::ApiError;

#[derive(Clone)]
pub struct ApiClient {
    base_url: String,
    http: reqwest::Client,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Result<Self, ApiError> {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()?;
        Ok(Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http,
        })
    }

    pub fn from_env() -> Result<Self, ApiError> {
        let url = std::env::var("DANQING_API_URL").unwrap_or_else(|_| "http://127.0.0.1:7800".into());
        Self::new(url)
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    // ------------------------------------------------------------------
    // Health / system
    // ------------------------------------------------------------------

    pub async fn health(&self) -> Result<serde_json::Value, ApiError> {
        self.get("/api/system/health").await
    }

    pub async fn metrics(&self) -> Result<serde_json::Value, ApiError> {
        self.get("/api/system/metrics").await
    }

    // ------------------------------------------------------------------
    // Models / registry
    // ------------------------------------------------------------------

    pub async fn list_models(&self) -> Result<Vec<serde_json::Value>, ApiError> {
        let resp: serde_json::Value = self.get("/api/models").await?;
        Ok(resp.as_array().cloned().unwrap_or_default())
    }

    pub async fn get_registry(&self) -> Result<serde_json::Value, ApiError> {
        self.get("/api/registry").await
    }

    // ------------------------------------------------------------------
    // Image generation
    // ------------------------------------------------------------------

    pub async fn create_image_generation<B: Serialize>(
        &self,
        body: &B,
    ) -> Result<serde_json::Value, ApiError> {
        self.post("/api/images/generations", body).await
    }

    pub async fn create_image_edit<B: Serialize>(
        &self,
        body: &B,
    ) -> Result<serde_json::Value, ApiError> {
        self.post("/api/images/edits", body).await
    }

    pub async fn create_image_upscale<B: Serialize>(
        &self,
        body: &B,
    ) -> Result<serde_json::Value, ApiError> {
        self.post("/api/images/upscales", body).await
    }

    // ------------------------------------------------------------------
    // Tasks
    // ------------------------------------------------------------------

    pub async fn get_task(&self, task_id: &str) -> Result<serde_json::Value, ApiError> {
        let path = format!("/api/tasks/{}", urlencoding::encode(task_id));
        self.get(&path).await
    }

    pub fn task_stream_url(&self, task_id: &str) -> String {
        format!("{}/api/tasks/{}/stream", self.base_url, urlencoding::encode(task_id))
    }

    pub async fn cancel_task(&self, task_id: &str) -> Result<(), ApiError> {
        let path = format!("/api/tasks/{}", urlencoding::encode(task_id));
        let resp = self.http.delete(&format!("{}{}", self.base_url, path)).send().await?;
        if !resp.status().is_success() {
            return Err(ApiError::Http(resp.status().as_u16(), resp.text().await.unwrap_or_default()));
        }
        Ok(())
    }

    // ------------------------------------------------------------------
    // Assets
    // ------------------------------------------------------------------

    pub fn asset_file_url(&self, asset_id: &str) -> String {
        format!("{}/api/assets/{}/file", self.base_url, urlencoding::encode(asset_id))
    }

    pub fn asset_thumbnail_url(&self, asset_id: &str) -> String {
        format!("{}/api/assets/{}/thumbnail", self.base_url, urlencoding::encode(asset_id))
    }

    pub async fn upload_asset(&self, file_path: &std::path::Path, mime: &str) -> Result<serde_json::Value, ApiError> {
        let file_bytes = tokio::fs::read(file_path).await.map_err(|e| ApiError::Http(0, e.to_string()))?;
        let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("upload");
        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name.to_string())
            .mime_str(mime)
            .map_err(|e| ApiError::Http(0, e.to_string()))?;
        let form = reqwest::multipart::Form::new().part("file", part);
        let url = format!("{}/api/assets", self.base_url);
        let resp = self.http.post(&url).multipart(form).send().await?;
        if !resp.status().is_success() {
            return Err(ApiError::Http(resp.status().as_u16(), resp.text().await.unwrap_or_default()));
        }
        Ok(resp.json().await?)
    }

    pub async fn list_assets(
        &self,
        kind: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<serde_json::Value, ApiError> {
        let mut params = vec![
            ("limit", limit.to_string()),
            ("offset", offset.to_string()),
        ];
        if let Some(k) = kind {
            params.push(("kind", k.to_string()));
        }
        let url = format!("{}/api/assets", self.base_url);
        let resp = self.http.get(&url).query(&params).send().await?;
        if !resp.status().is_success() {
            return Err(ApiError::Http(resp.status().as_u16(), resp.text().await.unwrap_or_default()));
        }
        Ok(resp.json().await?)
    }

    pub async fn delete_asset(&self, asset_id: &str) -> Result<(), ApiError> {
        let path = format!("/api/assets/{}", urlencoding::encode(asset_id));
        let resp = self.http.delete(&format!("{}{}", self.base_url, path)).send().await?;
        if !resp.status().is_success() {
            return Err(ApiError::Http(resp.status().as_u16(), resp.text().await.unwrap_or_default()));
        }
        Ok(())
    }

    // ------------------------------------------------------------------
    // Download
    // ------------------------------------------------------------------

    pub async fn download_file(&self, url: &str, dest: &std::path::Path) -> Result<(), ApiError> {
        let resp = self.http.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(ApiError::Http(resp.status().as_u16(), resp.text().await.unwrap_or_default()));
        }
        let bytes = resp.bytes().await?;
        tokio::fs::write(dest, bytes).await.map_err(|e| ApiError::Http(0, e.to_string()))?;
        Ok(())
    }

    // ------------------------------------------------------------------
    // SSE stream
    // ------------------------------------------------------------------

    pub async fn stream_task_events(
        &self,
        task_id: &str,
        mut on_event: impl FnMut(&str, &serde_json::Value),
    ) -> Result<(), ApiError> {
        let url = self.task_stream_url(task_id);
        let resp = self.http
            .get(&url)
            .header(ACCEPT, "text/event-stream")
            .send()
            .await?;
        
        if !resp.status().is_success() {
            return Err(ApiError::Http(resp.status().as_u16(), resp.text().await.unwrap_or_default()));
        }

        let mut stream = resp.bytes_stream();
        let mut buf = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| ApiError::Http(0, e.to_string()))?;
            let text = String::from_utf8_lossy(&chunk);
            buf.push_str(&text);

            while let Some(pos) = buf.find("\n\n") {
                let block = buf.split_off(pos + 2);
                let event_text = std::mem::replace(&mut buf, block);
                Self::parse_sse_block(&event_text, &mut on_event);
            }
        }

        Ok(())
    }

    fn parse_sse_block(block: &str, on_event: &mut impl FnMut(&str, &serde_json::Value)) {
        let mut event_type = "message";
        let mut data_json = None;

        for line in block.lines() {
            if line.starts_with("event:") {
                event_type = line[6..].trim();
            } else if line.starts_with("data:") {
                let data = line[5..].trim();
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(data) {
                    data_json = Some(v);
                }
            }
        }

        if let Some(ref data) = data_json {
            on_event(event_type, data);
        }
    }

    // ------------------------------------------------------------------
    // Low-level helpers
    // ------------------------------------------------------------------

    pub async fn get<T: DeserializeOwned>(&self,
        path: &str,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(ApiError::Http(resp.status().as_u16(), resp.text().await.unwrap_or_default()));
        }
        Ok(resp.json().await?)
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.post(&url).json(body).send().await?;
        if !resp.status().is_success() {
            return Err(ApiError::Http(resp.status().as_u16(), resp.text().await.unwrap_or_default()));
        }
        Ok(resp.json().await?)
    }
}
