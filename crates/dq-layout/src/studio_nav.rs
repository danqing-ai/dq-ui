use dq_tokens::{layout, spacing};
use dq_theme::studio_nav_container;
use iced::widget::{container, row, Space};
use iced::{Alignment, Element, Length};

/// Minimal top bar: page title (left) | spacer.
pub fn dq_studio_nav<'a, Message: Clone + 'a>(
    page_title: Element<'a, Message>,
) -> Element<'a, Message> {
    let h = layout::STUDIO_NAV_HEIGHT as f32;

    container(
        row![
            container(page_title)
                .center_y(Length::Fill)
                .padding([0.0, spacing::MD]),
            Space::new().width(Length::Fill),
        ]
        .align_y(Alignment::Center)
        .height(Length::Fixed(h))
        .width(Length::Fill),
    )
    .height(Length::Fixed(h))
    .padding([0.0, spacing::LG])
    .width(Length::Fill)
    .style(studio_nav_container)
    .into()
}
