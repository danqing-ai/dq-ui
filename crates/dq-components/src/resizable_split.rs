use dq_tokens::color;
use iced::widget::{container, row};
use iced::{Alignment, Element, Length, Point, Rectangle, Renderer, Size, Theme};
use iced::widget::canvas::{self, Canvas};

/// State for the resizable split view.
#[derive(Debug, Clone)]
pub struct SplitViewState {
    pub split_ratio: f32, // 0.0 = all left, 1.0 = all right
    pub dragging: bool,
    pub min_ratio: f32,
    pub max_ratio: f32,
}

impl Default for SplitViewState {
    fn default() -> Self {
        Self {
            split_ratio: 0.6,
            dragging: false,
            min_ratio: 0.2,
            max_ratio: 0.8,
        }
    }
}

impl SplitViewState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self,
        msg: SplitViewMessage,
        bounds: Rectangle,
    ) {
        match msg {
            SplitViewMessage::DragStarted(position) => {
                self.dragging = true;
                self.update_split(position.x, bounds.width);
            }
            SplitViewMessage::DragMoved(position) => {
                if self.dragging {
                    self.update_split(position.x, bounds.width);
                }
            }
            SplitViewMessage::DragEnded => {
                self.dragging = false;
            }
        }
    }

    fn update_split(&mut self, x: f32, width: f32) {
        if width > 0.0 {
            self.split_ratio = (x / width).clamp(self.min_ratio, self.max_ratio);
        }
    }
}

/// Messages emitted by the split view.
#[derive(Debug, Clone)]
pub enum SplitViewMessage {
    DragStarted(Point),
    DragMoved(Point),
    DragEnded,
}

/// Resizable split view with draggable divider.
pub fn resizable_split_view<'a, Message: Clone + 'a>(
    state: &'a SplitViewState,
    left: Element<'a, Message>,
    right: Element<'a, Message>,
    on_message: impl Fn(SplitViewMessage) -> Message + Clone + 'a,
) -> Element<'a, Message> {
    row![
        container(left)
            .width(Length::FillPortion((state.split_ratio * 100.0) as u16))
            .height(Length::Fill),
        draggable_divider(state, on_message),
        container(right)
            .width(Length::FillPortion(((1.0 - state.split_ratio) * 100.0) as u16))
            .height(Length::Fill),
    ]
    .spacing(0.0)
    .align_y(Alignment::Start)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn draggable_divider<'a, Message: Clone + 'a>(
    state: &'a SplitViewState,
    on_message: impl Fn(SplitViewMessage) -> Message + Clone + 'a,
) -> Element<'a, Message> {
    Canvas::new(DividerProgram {
        state,
        on_message: Box::new(on_message),
    })
    .width(Length::Fixed(8.0))
    .height(Length::Fill)
    .into()
}

struct DividerProgram<'a, Message> {
    state: &'a SplitViewState,
    on_message: Box<dyn Fn(SplitViewMessage) -> Message + 'a>,
}

impl<'a, Message: Clone + 'a> canvas::Program<Message> for DividerProgram<'a, Message> {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // Draw visual divider line in center
        let center_x = bounds.width / 2.0;
        let line = canvas::Path::line(
            Point::new(center_x, 0.0),
            Point::new(center_x, bounds.height),
        );
        frame.stroke(
            &line,
            canvas::Stroke::default()
                .with_width(1.0)
                .with_color(color::separator()),
        );

        // Draw grip indicator (3 dots)
        let grip_y = bounds.height / 2.0;
        for i in -1..=1 {
            let dot = canvas::Path::circle(
                Point::new(center_x, grip_y + i as f32 * 4.0),
                1.5,
            );
            frame.fill(
                &dot,
                if self.state.dragging {
                    color::accent()
                } else {
                    color::text_tertiary()
                },
            );
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
                    let msg = (self.on_message)(SplitViewMessage::DragStarted(position));
                    return Some(iced::widget::Action::publish(msg));
                }
            }
            iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)) => {
                let msg = (self.on_message)(SplitViewMessage::DragEnded);
                return Some(iced::widget::Action::publish(msg));
            }
            iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                if cursor.is_over(bounds) || self.state.dragging {
                    let msg = (self.on_message)(SplitViewMessage::DragMoved(*position));
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
        _bounds: Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> iced::mouse::Interaction {
        if cursor.is_over(Rectangle::new(Point::new(0.0, 0.0), Size::new(8.0, 1000.0))) {
            iced::mouse::Interaction::ResizingHorizontally
        } else {
            iced::mouse::Interaction::default()
        }
    }
}
