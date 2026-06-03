use crate::icons::sparkle_icon;
use crate::progress::dq_progress_bar;
use dq_tokens::color;
use dq_theme::inset_panel;
use iced::widget::{column, container, row, stack, text, Space};
use iced::{Alignment, Color, Element, Length};

#[derive(Debug, Clone, Copy)]
pub enum PreviewState {
    Idle,
    Generating { progress: u8 },
    Done,
}

/// Abstract landscape placeholder — no external image assets.
pub fn dq_preview_canvas<'a, Message: 'a>(
    state: PreviewState,
    size: f32,
) -> Element<'a, Message> {
    let sky = Color::from_rgb8(0x14, 0x1a, 0x2e);
    let peak = Color::from_rgb8(0x24, 0x20, 0x3a);
    let ridge = Color::from_rgb8(0x18, 0x1c, 0x28);
    let ground = Color::from_rgb8(0x0f, 0x12, 0x10);
    let accent_mist = Color::from_rgb8(0x2a, 0x2e, 0x52);

    let sky_band = container(Space::new())
        .width(Length::Fill)
        .height(Length::FillPortion(50))
        .style(move |_theme| container::Style {
            background: Some(iced::Background::Color(sky)),
            ..Default::default()
        });

    let mountains = row![
        container(Space::new())
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .style(move |_theme| container::Style {
                background: Some(iced::Background::Color(ridge)),
                ..Default::default()
            }),
        container(Space::new())
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .style(move |_theme| container::Style {
                background: Some(iced::Background::Color(peak)),
                ..Default::default()
            }),
        container(Space::new())
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .style(move |_theme| container::Style {
                background: Some(iced::Background::Color(ridge)),
                ..Default::default()
            }),
    ]
    .width(Length::Fill)
    .height(Length::FillPortion(35));

    let ground_band = container(Space::new())
        .width(Length::Fill)
        .height(Length::FillPortion(15))
        .style(move |_theme| container::Style {
            background: Some(iced::Background::Color(ground)),
            ..Default::default()
        });

    let mut art_layers = column![sky_band, mountains, ground_band]
        .width(Length::Fill)
        .height(Length::Fill);

    if matches!(state, PreviewState::Done) {
        art_layers = art_layers.push(
            container(Space::new())
                .width(Length::Fill)
                .height(Length::Fixed(0.0))
                .style(move |_theme| container::Style {
                    background: Some(iced::Background::Color(accent_mist)),
                    ..Default::default()
                }),
        );
    }

    let art = container(art_layers)
        .width(Length::Fill)
        .height(Length::Fill);

    let overlay: Element<'a, Message> = match state {
        PreviewState::Idle => container(
            text("暂无预览")
                .size(dq_tokens::typography::CAPTION)
                .color(color::text_quaternary()),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fill)
        .into(),
        PreviewState::Generating { progress } => container(
            column![
                text(format!("生成中 {progress}%"))
                    .size(dq_tokens::typography::LABEL)
                    .color(color::text_primary()),
                dq_progress_bar(progress as f32 / 100.0, 3.0),
            ]
            .spacing(8.0)
            .width(Length::Fill)
            .max_width(168.0)
            .align_x(Alignment::Center),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fill)
        .into(),
        PreviewState::Done => container(
            column![
                container(sparkle_icon::<Message>(color::accent()))
                .padding([6.0, 10.0])
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgba(
                        0.0, 0.0, 0.0, 0.45,
                    ))),
                    border: iced::Border {
                        color: color::border_subtle(),
                        width: 1.0,
                        radius: dq_tokens::spacing::radius_control().into(),
                    },
                    ..Default::default()
                }),
                text("生成完成")
                    .size(dq_tokens::typography::CAPTION)
                    .color(color::text_secondary()),
            ]
            .spacing(6.0)
            .align_x(Alignment::Center),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fill)
        .into(),
    };

    container(
        stack![art, overlay]
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fixed(size))
    .height(Length::Fixed(size))
    .style(inset_panel)
    .clip(true)
    .into()
}
