#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("HTTP {0}: {1}")]
    Http(u16, String),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
