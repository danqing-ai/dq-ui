use crate::{dq_section, phosphor_icon, PhosphorIcon, SectionIcon};
use dq_tokens::{color, spacing, typography};
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub struct LogLine {
    pub time: String,
    pub message: String,
}

pub fn default_logs() -> Vec<LogLine> {
    vec![
        LogLine {
            time: "07:05:19".into(),
            message: "已切换模型：Z-Image-Turbo（fp16）".into(),
        },
        LogLine {
            time: "07:05:22".into(),
            message: "已加载预设：写实摄影".into(),
        },
        LogLine {
            time: "07:06:01".into(),
            message: "队列空闲 · 等待提交".into(),
        },
    ]
}

pub fn log_panel<'a, Message: Clone + 'a>(
    logs: &'a [LogLine],
    on_clear: Option<Message>,
) -> Element<'a, Message> {
    // Ghost icon button — no background, just the icon.
    let clear = button(phosphor_icon(PhosphorIcon::Trash, 14.0, color::text_tertiary()))
        .padding(4)
        .style(|_theme, _status| button::Style {
            background: None,
            ..Default::default()
        })
        .on_press_maybe(on_clear)
        .into();

    let body: Element<Message> = if logs.is_empty() {
        container(
            text("暂无日志")
                .size(typography::LABEL)
                .color(color::text_tertiary()),
        )
        .width(Length::Fill)
        .padding(spacing::MD)
        .align_x(Alignment::Center)
        .into()
    } else {
        let mut col = column![].spacing(spacing::XS).width(Length::Fill);
        for line in logs {
            col = col.push(
                row![
                    text(format!("[{}]", line.time))
                        .size(typography::MINI)
                        .color(color::text_quaternary()),
                    text(line.message.as_str())
                        .size(typography::CAPTION)
                        .color(color::accent()),
                ]
                .spacing(spacing::SM)
                .align_y(Alignment::Start)
                .width(Length::Fill),
            );
        }
        container(col)
            .padding(spacing::SM)
            .width(Length::Fill)
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
    };

    dq_section(SectionIcon::Document, "日志", Some(clear), Some(body))
}
