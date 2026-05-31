use dq_components::{brand_logo, studio_icon, StudioIcon};
use dq_tokens::{color, layout, spacing};
use dq_theme::sidebar_container;
use iced::widget::{button, column, container, text, tooltip, Space};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub struct NavItem<Message> {
    pub id: String,
    pub icon: StudioIcon,
    pub label: String,
    pub message: Message,
}

#[derive(Debug, Clone)]
pub struct SidebarSection<Message> {
    pub label: Option<String>,
    pub items: Vec<NavItem<Message>>,
}

/// Icon-only sidebar (56px) — Linear-style minimal chrome.
/// Logo at top, settings at very bottom, task queue above settings.
pub fn dq_sidebar<'a, Message>(
    sections: Vec<SidebarSection<Message>>,
    active_id: &str,
    on_task_queue: Message,
    on_settings: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut nav_col = column![].spacing(spacing::XS).width(Length::Fill);

    // Logo at top
    nav_col = nav_col.push(
        container(brand_logo::<Message>())
            .width(Length::Fill)
            .height(Length::Fixed(44.0))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .padding(spacing::XS),
    );

    nav_col = nav_col.push(
        container(Space::new())
            .width(Length::Fill)
            .height(Length::Fixed(1.0))
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(color::SEPARATOR)),
                ..Default::default()
            }),
    );

    // Navigation items
    for section in sections {
        for item in section.items {
            nav_col = nav_col.push(nav_item_view(item, active_id));
        }
    }

    nav_col = nav_col.push(Space::new().height(Length::Fill));

    // Bottom section: task queue + settings
    nav_col = nav_col.push(
        container(Space::new())
            .width(Length::Fill)
            .height(Length::Fixed(1.0))
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(color::SEPARATOR)),
                ..Default::default()
            }),
    );

    // Task queue button — icon-only, matching nav style
    let queue_active = false;
    let queue_icon = studio_icon(StudioIcon::Queue, queue_active);
    let queue_btn = button(
        container(queue_icon)
            .width(Length::Fixed(36.0))
            .height(Length::Fixed(36.0))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
    )
    .width(Length::Fixed(40.0))
    .height(Length::Fixed(40.0))
    .padding(0.0)
    .style(dq_components::nav_item_button(queue_active))
    .on_press(on_task_queue);

    nav_col = nav_col.push(
        container(
            tooltip(
                queue_btn,
                text("任务队列").size(11),
                iced::widget::tooltip::Position::Right,
            )
        )
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(spacing::XS),
    );

    // Settings icon at very bottom
    let settings_active = active_id == "settings";
    let settings_icon = studio_icon(StudioIcon::Settings, settings_active);
    let settings_btn = button(
        container(settings_icon)
            .width(Length::Fixed(36.0))
            .height(Length::Fixed(36.0))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
    )
    .width(Length::Fixed(40.0))
    .height(Length::Fixed(40.0))
    .padding(0.0)
    .style(dq_components::nav_item_button(settings_active))
    .on_press(on_settings);

    nav_col = nav_col.push(
        container(
            tooltip(
                settings_btn,
                text("设置").size(11),
                iced::widget::tooltip::Position::Right,
            )
        )
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(spacing::XS),
    );

    container(nav_col)
        .width(Length::Fixed(layout::SIDEBAR_WIDTH as f32))
        .height(Length::Fill)
        .padding([spacing::SM, spacing::XS])
        .style(sidebar_container)
        .into()
}

fn nav_item_view<'a, Message: Clone + 'a>(
    item: NavItem<Message>,
    active_id: &str,
) -> Element<'a, Message> {
    let active = item.id == active_id;

    let icon = studio_icon(item.icon, active);

    let btn = button(
        container(icon)
            .width(Length::Fixed(36.0))
            .height(Length::Fixed(36.0))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
    )
    .width(Length::Fixed(40.0))
    .height(Length::Fixed(40.0))
    .padding(0.0)
    .style(dq_components::nav_item_button(active))
    .on_press(item.message);

    tooltip(
        btn,
        text(item.label).size(11),
        iced::widget::tooltip::Position::Right,
    )
    .into()
}


