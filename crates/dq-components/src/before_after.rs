use dq_tokens::color;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::widget::{container, row, text};
use iced::{Alignment, Color, Element, Length, Point, Rectangle, Renderer, Size, Theme, Vector};

/// State for the before/after slider.
#[derive(Debug, Clone)]
pub struct BeforeAfterState {
    pub split_ratio: f32, // 0.0 = all before, 1.0 = all after
    pub dragging: bool,
}

impl Default for BeforeAfterState {
    fn default() -> Self {
        Self {
            split_ratio: 0.5,
            dragging: false,
        }
    }
}

impl BeforeAfterState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self,
        msg: BeforeAfterMessage,
        bounds: Rectangle,
    ) {
        match msg {
            BeforeAfterMessage::DragStarted(position) => {
                self.dragging = true;
                self.update_split(position.x, bounds.width);
            }
            BeforeAfterMessage::DragMoved(position) => {
                if self.dragging {
                    self.update_split(position.x, bounds.width);
                }
            }
            BeforeAfterMessage::DragEnded => {
                self.dragging = false;
            }
            BeforeAfterMessage::SetRatio(ratio) => {
                self.split_ratio = ratio.clamp(0.0, 1.0);
            }
        }
    }

    fn update_split(&mut self, x: f32, width: f32) {
        if width > 0.0 {
            self.split_ratio = (x / width).clamp(0.0, 1.0);
        }
    }
}

/// Messages emitted by the before/after slider.
#[derive(Debug, Clone)]
pub enum BeforeAfterMessage {
    DragStarted(Point),
    DragMoved(Point),
    DragEnded,
    SetRatio(f32),
}

/// Build the before/after comparison slider.
pub fn before_after_slider<'a, Message: Clone + 'a>(
    state: &'a BeforeAfterState,
    before_label: &'static str,
    after_label: &'static str,
    on_message: impl Fn(BeforeAfterMessage) -> Message + Clone + 'a,
) -> Element<'a, Message> {
    let canvas_widget = Canvas::new(BeforeAfterProgram {
        state,
        before_label,
        after_label,
        on_message: Box::new(on_message),
    })
    .width(Length::Fill)
    .height(Length::Fill);

    container(canvas_widget)
        .width(Length::Fill)
        .height(Length::Fixed(300.0))
        .style(|_theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(color::BG_INSET)),
            border: iced::Border {
                color: color::BORDER_SUBTLE,
                width: 1.0,
                radius: dq_tokens::spacing::RADIUS_MD.into(),
            },
            ..Default::default()
        })
        .into()
}

struct BeforeAfterProgram<'a, Message> {
    state: &'a BeforeAfterState,
    before_label: &'static str,
    after_label: &'static str,
    on_message: Box<dyn Fn(BeforeAfterMessage) -> Message + 'a>,
}

impl<'a, Message: Clone + 'a> canvas::Program<Message> for BeforeAfterProgram<'a, Message> {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let split_x = bounds.width * self.state.split_ratio;

        // Draw "before" side (left, darker)
        let before_rect = Path::rectangle(
            Point::new(0.0, 0.0),
            Size::new(split_x, bounds.height),
        );
        frame.fill(
            &before_rect,
            Color::from_rgba(0.15, 0.15, 0.18, 1.0),
        );

        // Draw "after" side (right, lighter)
        let after_rect = Path::rectangle(
            Point::new(split_x, 0.0),
            Size::new(bounds.width - split_x, bounds.height),
        );
        frame.fill(
            &after_rect,
            Color::from_rgba(0.22, 0.22, 0.26, 1.0),
        );

        // Draw divider line
        let divider = Path::line(
            Point::new(split_x, 0.0),
            Point::new(split_x, bounds.height),
        );
        frame.stroke(
            &divider,
            Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.8)),
        );

        // Draw divider handle (circle)
        let handle_y = bounds.height / 2.0;
        let handle = Path::circle(Point::new(split_x, handle_y), 12.0);
        frame.fill(
            &handle,
            Color::from_rgba(1.0, 1.0, 1.0, 0.9),
        );
        frame.stroke(
            &handle,
            Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgba(0.3, 0.3, 0.3, 1.0)),
        );

        // Draw arrows inside handle
        let arrow_color = Color::from_rgba(0.3, 0.3, 0.3, 1.0);
        let arrow_left = Path::line(
            Point::new(split_x - 4.0, handle_y),
            Point::new(split_x - 8.0, handle_y),
        );
        let arrow_right = Path::line(
            Point::new(split_x + 4.0, handle_y),
            Point::new(split_x + 8.0, handle_y),
        );
        frame.stroke(
            &arrow_left,
            Stroke::default().with_width(1.5).with_color(arrow_color),
        );
        frame.stroke(
            &arrow_right,
            Stroke::default().with_width(1.5).with_color(arrow_color),
        );

        // Draw labels
        let label_color = Color::from_rgba(0.7, 0.7, 0.7, 0.8);
        // We can't easily draw text in canvas Frame in iced 0.14 without a font,
        // so we'll skip text rendering in the canvas and use overlay widgets instead

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
                    let msg = (self.on_message)(BeforeAfterMessage::DragStarted(position));
                    return Some(iced::widget::Action::publish(msg));
                }
            }
            iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)) => {
                let msg = (self.on_message)(BeforeAfterMessage::DragEnded);
                return Some(iced::widget::Action::publish(msg));
            }
            iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                if cursor.is_over(bounds) {
                    let msg = (self.on_message)(BeforeAfterMessage::DragMoved(*position));
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
            iced::mouse::Interaction::ResizingHorizontally
        } else {
            iced::mouse::Interaction::default()
        }
    }
}

/// Label overlay for the before/after slider.
pub fn before_after_labels<'a, Message: Clone + 'a>(
    before_label: &'static str,
    after_label: &'static str,
) -> Element<'a, Message> {
    row![
        container(text(before_label).size(dq_tokens::typography::LABEL))
            .width(Length::FillPortion(1))
            .align_x(Alignment::Start),
        container(text(after_label).size(dq_tokens::typography::LABEL))
            .width(Length::FillPortion(1))
            .align_x(Alignment::End),
    ]
    .spacing(dq_tokens::spacing::SM)
    .width(Length::Fill)
    .into()
}
