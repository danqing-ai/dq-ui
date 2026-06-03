use dq_tokens::color;
use iced::widget::canvas::{self, Cache, Canvas, Geometry, Path, Stroke};
use iced::widget::{button, row, text};
use iced::{Alignment, Color, Length, Point, Rectangle, Renderer, Size, Theme, Vector};

/// Tool mode for the canvas editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CanvasTool {
    #[default]
    Pan,
    Brush,
    Eraser,
}

/// A single brush stroke.
#[derive(Debug, Clone)]
pub struct StrokeData {
    pub points: Vec<Point>,
    pub width: f32,
    pub is_eraser: bool,
}

/// Exported mask region.
#[derive(Debug, Clone)]
pub struct MaskRegion {
    pub points: Vec<Point>,
    pub width: f32,
}

/// Messages emitted by the canvas editor.
#[derive(Debug, Clone)]
pub enum CanvasEditorMessage {
    MousePressed(Point),
    MouseDragged(Point),
    MouseReleased,
    ZoomIn,
    ZoomOut,
    ResetView,
    ClearMask,
    SetTool(CanvasTool),
    SetBrushSize(f32),
}

/// State for the layered canvas editor.
#[derive(Debug)]
pub struct CanvasEditorState {
    pub zoom: f32,
    pub pan: Vector,
    pub tool: CanvasTool,
    pub brush_size: f32,
    pub strokes: Vec<StrokeData>,
    pub current_stroke: Option<StrokeData>,
    pub base_image_path: Option<std::path::PathBuf>,
    last_pan_pos: Option<Point>,
    cache: Cache,
    pub bounds: Size,
}

impl Clone for CanvasEditorState {
    fn clone(&self) -> Self {
        Self {
            zoom: self.zoom,
            pan: self.pan,
            tool: self.tool,
            brush_size: self.brush_size,
            strokes: self.strokes.clone(),
            current_stroke: self.current_stroke.clone(),
            base_image_path: self.base_image_path.clone(),
            last_pan_pos: None,
            cache: Cache::new(),
            bounds: self.bounds,
        }
    }
}

impl Default for CanvasEditorState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan: Vector::new(0.0, 0.0),
            tool: CanvasTool::Pan,
            brush_size: 25.0,
            strokes: Vec::new(),
            current_stroke: None,
            base_image_path: None,
            last_pan_pos: None,
            cache: Cache::new(),
            bounds: Size::new(512.0, 512.0),
        }
    }
}

impl CanvasEditorState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self,
        msg: CanvasEditorMessage,
    ) {
        match msg {
            CanvasEditorMessage::MousePressed(point) => {
                if matches!(self.tool, CanvasTool::Brush | CanvasTool::Eraser) {
                    self.current_stroke = Some(StrokeData {
                        points: vec![point],
                        width: self.brush_size / self.zoom,
                        is_eraser: self.tool == CanvasTool::Eraser,
                    });
                } else if matches!(self.tool, CanvasTool::Pan) {
                    self.last_pan_pos = Some(point);
                }
            }
            CanvasEditorMessage::MouseDragged(point) => {
                if matches!(self.tool, CanvasTool::Pan) {
                    if let Some(last) = self.last_pan_pos {
                        let delta = Vector::new(point.x - last.x, point.y - last.y);
                        self.pan = self.pan + delta;
                        self.last_pan_pos = Some(point);
                        self.cache.clear();
                    }
                } else if let Some(ref mut stroke) = self.current_stroke {
                    stroke.points.push(point);
                }
            }
            CanvasEditorMessage::MouseReleased => {
                self.last_pan_pos = None;
                if let Some(stroke) = self.current_stroke.take() {
                    if stroke.points.len() >= 2 {
                        self.strokes.push(stroke);
                    }
                }
                self.cache.clear();
            }
            CanvasEditorMessage::ZoomIn => {
                self.zoom = (self.zoom * 1.2).clamp(0.1, 10.0);
                self.cache.clear();
            }
            CanvasEditorMessage::ZoomOut => {
                self.zoom = (self.zoom / 1.2).clamp(0.1, 10.0);
                self.cache.clear();
            }
            CanvasEditorMessage::ResetView => {
                self.zoom = 1.0;
                self.pan = Vector::new(0.0, 0.0);
                self.cache.clear();
            }
            CanvasEditorMessage::ClearMask => {
                self.strokes.clear();
                self.current_stroke = None;
                self.cache.clear();
            }
            CanvasEditorMessage::SetTool(tool) => {
                self.tool = tool;
            }
            CanvasEditorMessage::SetBrushSize(size) => {
                self.brush_size = size.clamp(1.0, 200.0);
            }
        }
    }

    pub fn export_mask_regions(&self) -> Vec<MaskRegion> {
        self.strokes
            .iter()
            .filter(|s| !s.is_eraser)
            .map(|s| MaskRegion {
                points: s.points.clone(),
                width: s.width,
            })
            .collect()
    }
}

// ─── Canvas Widget ───────────────────────────────────────────────────────────

/// Build the canvas widget with optional base image underneath brush overlay.
pub fn canvas_editor<'a, Message: Clone + 'a>(
    state: &'a CanvasEditorState,
    on_message: impl Fn(CanvasEditorMessage) -> Message + Clone + 'a,
) -> iced::Element<'a, Message> {
    use iced::widget::{image, Stack};

    let canvas_widget = Canvas::new(CanvasProgram {
        state,
        on_message: Box::new(on_message),
    })
    .width(Length::Fill)
    .height(Length::Fill);

    // If there's a base image, layer canvas on top of it
    if let Some(ref path) = state.base_image_path {
        if path.exists() {
            let handle = image::Handle::from_path(path);
            Stack::new()
                .push(
                    image(handle)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(iced::ContentFit::Contain)
                )
                .push(canvas_widget)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
            canvas_widget.into()
        }
    } else {
        canvas_widget.into()
    }
}

struct CanvasProgram<'a, Message> {
    state: &'a CanvasEditorState,
    on_message: Box<dyn Fn(CanvasEditorMessage) -> Message + 'a>,
}

impl<'a, Message: Clone + 'a> canvas::Program<Message> for CanvasProgram<'a, Message> {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // Apply pan offset
        frame.translate(self.state.pan);

        // Draw dashed boundary rectangle
        draw_dashed_boundary(&mut frame, bounds);

        // Draw center crosshair for alignment
        draw_crosshair(&mut frame, bounds);

        // Draw all committed strokes
        for stroke in &self.state.strokes {
            draw_stroke(&mut frame, stroke);
        }

        // Draw current in-progress stroke
        if let Some(ref stroke) = self.state.current_stroke {
            draw_stroke(&mut frame, stroke);
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut (),
        event: &iced::Event,
        bounds: Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> Option<iced::widget::Action<Message>> {
        match event {
            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
                if let Some(position) = cursor.position_in(bounds) {
                    let msg = (self.on_message)(CanvasEditorMessage::MousePressed(position));
                    return Some(iced::widget::Action::publish(msg));
                }
            }
            iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)) => {
                let msg = (self.on_message)(CanvasEditorMessage::MouseReleased);
                return Some(iced::widget::Action::publish(msg));
            }
            iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                if cursor.is_over(bounds) {
                    let msg = (self.on_message)(CanvasEditorMessage::MouseDragged(*position));
                    return Some(iced::widget::Action::publish(msg));
                }
            }
            _ => {}
        }

        None
    }

    fn mouse_interaction(
        &self,
        _state: &(),
        bounds: Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> iced::mouse::Interaction {
        if cursor.is_over(bounds) {
            match self.state.tool {
                CanvasTool::Pan => iced::mouse::Interaction::Grab,
                CanvasTool::Brush | CanvasTool::Eraser => iced::mouse::Interaction::Crosshair,
            }
        } else {
            iced::mouse::Interaction::default()
        }
    }
}

fn draw_stroke(frame: &mut canvas::Frame, stroke: &StrokeData) {
    if stroke.points.len() < 2 {
        return;
    }

    let path = Path::new(|builder| {
        builder.move_to(stroke.points[0]);
        for point in &stroke.points[1..] {
            builder.line_to(*point);
        }
    });

    let color = if stroke.is_eraser {
        Color::from_rgba(0.0, 0.0, 0.0, 0.0)
    } else {
        Color::from_rgba(1.0, 0.2, 0.2, 0.6)
    };

    frame.stroke(
        &path,
        Stroke::default()
            .with_width(stroke.width)
            .with_line_cap(canvas::LineCap::Round)
            .with_line_join(canvas::LineJoin::Round)
            .with_color(color),
    );
}

/// Draw a dashed boundary rectangle around the canvas area.
fn draw_dashed_boundary(frame: &mut canvas::Frame, bounds: Rectangle) {
    let margin = 20.0;
    let top_left = Point::new(margin, margin);
    let size = Size::new(
        bounds.width - margin * 2.0,
        bounds.height - margin * 2.0,
    );

    // Draw dashed rectangle using multiple line segments
    let dash_len = 8.0;
    let gap_len = 4.0;
    let color = Color::from_rgba(0.7, 0.7, 0.7, 0.5);
    let stroke_style = Stroke::default()
        .with_width(1.0)
        .with_color(color);

    let corners = [
        Point::new(top_left.x, top_left.y),
        Point::new(top_left.x + size.width, top_left.y),
        Point::new(top_left.x + size.width, top_left.y + size.height),
        Point::new(top_left.x, top_left.y + size.height),
    ];

    for i in 0..4 {
        let start = corners[i];
        let end = corners[(i + 1) % 4];
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let len = (dx * dx + dy * dy).sqrt();
        let unit_x = dx / len;
        let unit_y = dy / len;

        let mut pos = 0.0;
        while pos < len {
            let seg_start = pos.min(len);
            let seg_end = (pos + dash_len).min(len);
            if seg_start < seg_end {
                let p1 = Point::new(
                    start.x + unit_x * seg_start,
                    start.y + unit_y * seg_start,
                );
                let p2 = Point::new(
                    start.x + unit_x * seg_end,
                    start.y + unit_y * seg_end,
                );
                let path = Path::line(p1, p2);
                frame.stroke(&path, stroke_style);
            }
            pos += dash_len + gap_len;
        }
    }
}

/// Draw a center crosshair for alignment reference.
fn draw_crosshair(frame: &mut canvas::Frame, bounds: Rectangle) {
    let center_x = bounds.width / 2.0;
    let center_y = bounds.height / 2.0;
    let crosshair_len = 12.0;
    let color = Color::from_rgba(0.5, 0.5, 0.5, 0.3);
    let stroke_style = Stroke::default()
        .with_width(1.0)
        .with_color(color);

    // Horizontal line
    let h_path = Path::line(
        Point::new(center_x - crosshair_len, center_y),
        Point::new(center_x + crosshair_len, center_y),
    );
    frame.stroke(&h_path, stroke_style);

    // Vertical line
    let v_path = Path::line(
        Point::new(center_x, center_y - crosshair_len),
        Point::new(center_x, center_y + crosshair_len),
    );
    frame.stroke(&v_path, stroke_style);
}

// ─── Control Widgets ─────────────────────────────────────────────────────────

/// Brush size selector widget.
pub fn brush_size_selector<'a, Message: Clone + 'a>(
    current_size: f32,
    on_change: impl Fn(f32) -> Message + 'a,
) -> iced::Element<'a, Message> {
    row![
        text("画笔大小").size(dq_tokens::typography::LABEL).color(color::TEXT_SECONDARY),
        iced::widget::slider(1.0..=100.0, current_size, on_change)
            .width(Length::Fixed(120.0)),
        text(format!("{:.0}px", current_size))
            .size(dq_tokens::typography::CAPTION)
            .color(color::TEXT_TERTIARY),
    ]
    .spacing(dq_tokens::spacing::SM)
    .align_y(Alignment::Center)
    .into()
}

/// Tool selector buttons (Pan / Brush / Eraser).
pub fn tool_selector<'a, Message: Clone + 'a>(
    current: CanvasTool,
    on_select: impl Fn(CanvasTool) -> Message + Clone + 'a,
) -> iced::Element<'a, Message> {
    let tools = vec![
        (CanvasTool::Pan, "平移"),
        (CanvasTool::Brush, "画笔"),
        (CanvasTool::Eraser, "橡皮"),
    ];

    let mut buttons = row![].spacing(dq_tokens::spacing::XS);

    for (tool, label) in tools {
        let active = current == tool;
        let btn = button(
            text(label)
                .size(dq_tokens::typography::CAPTION)
                .color(if active { color::TEXT_PRIMARY } else { color::TEXT_TERTIARY }),
        )
        .padding([4.0, 12.0])
        .style(move |_theme: &Theme, _status: button::Status| button::Style {
            background: if active {
                Some(iced::Background::Color(color::FILL_SELECTED))
            } else {
                None
            },
            border: iced::Border {
                color: if active { color::BORDER_SUBTLE } else { Color::TRANSPARENT },
                width: 1.0,
                radius: dq_tokens::spacing::RADIUS_SM.into(),
            },
            ..Default::default()
        })
        .on_press((on_select)(tool));

        buttons = buttons.push(btn);
    }

    buttons.into()
}

/// Zoom controls (+ / − / reset / clear).
pub fn zoom_controls<'a, Message: Clone + 'a>(
    on_zoom_in: Message,
    on_zoom_out: Message,
    on_reset: Message,
    on_clear: Message,
) -> iced::Element<'a, Message> {
    use crate::phosphor::{phosphor_icon, PhosphorIcon};

    row![
        button(phosphor_icon(PhosphorIcon::MagnifyingGlassPlus, 14.0, color::TEXT_SECONDARY))
            .padding(4)
            .style(tool_button_style)
            .on_press(on_zoom_in),
        button(phosphor_icon(PhosphorIcon::MagnifyingGlass, 14.0, color::TEXT_SECONDARY))
            .padding(4)
            .style(tool_button_style)
            .on_press(on_reset),
        button(phosphor_icon(PhosphorIcon::Minus, 14.0, color::TEXT_SECONDARY))
            .padding(4)
            .style(tool_button_style)
            .on_press(on_zoom_out),
        button(phosphor_icon(PhosphorIcon::Trash, 14.0, color::TEXT_SECONDARY))
            .padding(4)
            .style(tool_button_style)
            .on_press(on_clear),
    ]
    .spacing(dq_tokens::spacing::XS)
    .into()
}

fn tool_button_style(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(iced::Background::Color(color::BG_INSET)),
        border: iced::Border {
            color: color::BORDER_SUBTLE,
            width: 1.0,
            radius: dq_tokens::spacing::RADIUS_SM.into(),
        },
        ..Default::default()
    }
}
