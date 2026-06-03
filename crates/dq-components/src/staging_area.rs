use dq_tokens::{color, spacing, typography};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Element, Length};

/// A single result item in the staging area.
#[derive(Debug, Clone)]
pub struct StagedResult {
    pub id: usize,
    pub title: String,
    pub selected: bool,
}

/// Messages for the staging area.
#[derive(Debug, Clone)]
pub enum StagingMessage {
    SelectResult(usize),
    DownloadResult(usize),
    DeleteResult(usize),
    ClearAll,
}

/// Build the generation staging area grid.
pub fn staging_area<'a, Message: Clone + 'a>(
    results: &'a [StagedResult],
    on_message: impl Fn(StagingMessage) -> Message + Clone + 'a,
) -> Element<'a, Message> {
    if results.is_empty() {
        return container(
            column![
                text("暂存区")
                    .size(typography::LABEL)
                    .color(color::text_secondary()),
                text("批量生成的图片将显示在这里")
                    .size(typography::CAPTION)
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
        .into();
    }

    let mut grid = row![].spacing(spacing::SM).width(Length::Fill);
    
    for result in results.iter().take(4) {
        grid = grid.push(staging_item(result, &on_message));
    }

    let header = row![
        text(format!("暂存区 · {} 张", results.len()))
            .size(typography::LABEL)
            .color(color::text_secondary()),
        Space::new().width(Length::Fill),
        button(
            text("清空")
                .size(typography::CAPTION)
                .color(color::text_tertiary()),
        )
        .padding([2.0, 8.0])
        .style(|_theme, _status| iced::widget::button::Style {
            background: None,
            ..Default::default()
        })
        .on_press((on_message)(StagingMessage::ClearAll)),
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill);

    column![
        header,
        grid,
    ]
    .spacing(spacing::SM)
    .width(Length::Fill)
    .into()
}

fn staging_item<'a, Message: Clone + 'a>(
    result: &'a StagedResult,
    on_message: &(impl Fn(StagingMessage) -> Message + Clone + 'a),
) -> Element<'a, Message> {
    use crate::phosphor::{phosphor_icon, PhosphorIcon};

    let border_color = if result.selected {
        color::accent()
    } else {
        color::border_subtle()
    };

    container(
        button(
            column![
                phosphor_icon(PhosphorIcon::Image, 24.0, color::text_quaternary()),
                text(result.title.as_str())
                    .size(typography::MINI)
                    .color(color::text_tertiary()),
            ]
            .spacing(spacing::XS)
            .align_x(Alignment::Center),
        )
        .padding(spacing::SM)
        .width(Length::Fill)
        .height(Length::Fixed(100.0))
        .style(move |_theme, _status| iced::widget::button::Style {
            background: Some(iced::Background::Color(color::bg_inset())),
            border: iced::Border {
                color: border_color,
                width: if result.selected { 2.0 } else { 1.0 },
                radius: spacing::radius_control_sm().into(),
            },
            ..Default::default()
        })
        .on_press((on_message)(StagingMessage::SelectResult(result.id))),
    )
    .width(Length::FillPortion(1))
    .into()
}
