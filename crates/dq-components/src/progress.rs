use dq_tokens::color;
use iced::widget::{container, row, Space};
use iced::{Element, Length};

/// Linear-style thin progress track with accent fill.
pub fn dq_progress_bar<'a, Message: 'a>(ratio: f32, height: f32) -> Element<'a, Message> {
    dq_progress_bar_colored(ratio, height, color::ACCENT)
}

/// Muted track — secondary metrics (VRAM, etc.).
pub fn dq_progress_bar_muted<'a, Message: 'a>(ratio: f32, height: f32) -> Element<'a, Message> {
    dq_progress_bar_colored(ratio, height, color::BG_SURFACE)
}

fn dq_progress_bar_colored<'a, Message: 'a>(
    ratio: f32,
    height: f32,
    fill_color: iced::Color,
) -> Element<'a, Message> {
    let clamped = ratio.clamp(0.0, 1.0);
    let filled = ((clamped * 1000.0).round() as u16).max(if clamped > 0.0 { 1 } else { 0 });
    let empty = 1000_u16.saturating_sub(filled).max(1);

    let radius = (height / 2.0).into();

    let fill = container(Space::new())
        .width(Length::FillPortion(filled.max(1)))
        .height(Length::Fixed(height))
        .style(move |_theme| container::Style {
            background: Some(iced::Background::Color(fill_color)),
            border: iced::Border {
                radius,
                ..Default::default()
            },
            ..Default::default()
        });

    container(
        row![
            fill,
            Space::new().width(Length::FillPortion(empty)),
        ]
        .width(Length::Fill)
        .height(Length::Fixed(height)),
    )
    .width(Length::Fill)
    .height(Length::Fixed(height))
    .style(move |_theme| container::Style {
        background: Some(iced::Background::Color(color::BG_SURFACE)),
        border: iced::Border {
            radius,
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}
