use dq_tokens::{color, spacing, typography};
use iced::widget::{container, image, text, column};
use iced::{Alignment, Element, Length};
use std::path::PathBuf;

/// Image viewer that shows an actual image or a placeholder.
pub fn image_viewer<'a, Message: Clone + 'a>(
    path: Option<&PathBuf>,
    placeholder_label: &'a str,
) -> Element<'a, Message> {
    match path {
        Some(path) if path.exists() => {
            let handle = image::Handle::from_path(path);
            container(
                image(handle)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(iced::ContentFit::Contain),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(color::bg_inset())),
                border: iced::Border {
                    color: color::border_subtle(),
                    width: 1.0,
                    radius: spacing::radius_control().into(),
                },
                ..Default::default()
            })
            .into()
        }
        _ => {
            container(
                column![
                    text(placeholder_label)
                        .size(typography::BODY)
                        .color(color::text_tertiary()),
                ]
                .spacing(spacing::XS)
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fixed(120.0))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(|_theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(color::bg_inset())),
                border: iced::Border {
                    color: color::border_subtle(),
                    width: 1.0,
                    radius: spacing::radius_control().into(),
                },
                ..Default::default()
            })
            .into()
        }
    }
}

/// Image preview with metadata overlay.
pub fn image_preview_with_meta<'a, Message: Clone + 'a>(
    path: Option<&PathBuf>,
    meta: &'a str,
    width: Length,
    height: Length,
) -> Element<'a, Message> {
    match path {
        Some(path) if path.exists() => {
            let handle = image::Handle::from_path(path);
            container(
                column![
                    image(handle)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(iced::ContentFit::Contain),
                    text(meta)
                        .size(typography::CAPTION)
                        .color(color::text_quaternary()),
                ]
                .spacing(spacing::XS)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            )
            .width(width)
            .height(height)
            .style(|_theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(color::bg_inset())),
                border: iced::Border {
                    color: color::border_subtle(),
                    width: 1.0,
                    radius: spacing::radius_control().into(),
                },
                ..Default::default()
            })
            .into()
        }
        _ => {
            container(
                column![
                    text(meta)
                        .size(typography::CAPTION)
                        .color(color::text_quaternary()),
                ]
                .spacing(spacing::XS)
                .align_x(Alignment::Center),
            )
            .width(width)
            .height(height)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(|_theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(color::bg_inset())),
                border: iced::Border {
                    color: color::border_subtle(),
                    width: 1.0,
                    radius: spacing::radius_control().into(),
                },
                ..Default::default()
            })
            .into()
        }
    }
}
