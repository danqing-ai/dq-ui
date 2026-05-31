use crate::phosphor::{phosphor_icon, PhosphorIcon};
use crate::styles::slider_style;
use dq_tokens::{color, spacing, typography};
use iced::widget::{button, container, row, slider, text};
use iced::{Alignment, Element, Length};

pub fn dq_slider<'a, Message: Clone + 'a>(
    range: std::ops::RangeInclusive<f32>,
    step: f32,
    value: f32,
    on_change: impl Fn(f32) -> Message + 'a,
) -> Element<'a, Message> {
    container(
        slider(range, value, on_change)
            .step(step)
            .width(Length::Fill)
            .style(slider_style),
    )
    .height(Length::Fixed(spacing::CONTROL_HEIGHT))
    .width(Length::Fill)
    .align_y(Alignment::Center)
    .into()
}

/// Modern numeric stepper: [-] [slider] [value] [+]
/// Provides intuitive fine-grained control alongside direct text input.
pub fn dq_slider_with_input<'a, Message: Clone + 'a>(
    range: std::ops::RangeInclusive<f32>,
    step: f32,
    value: f32,
    _input_text: &'a str,
    on_slider_change: impl Fn(f32) -> Message + Clone + 'a,
    _on_input_change: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    let slider_elem = dq_slider(range.clone(), step, value, on_slider_change.clone());

    // Minus button
    let min = *range.start();
    let max = *range.end();
    let minus_btn = button(
        container(phosphor_icon(PhosphorIcon::Minus, 10.0, color::TEXT_SECONDARY))
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .on_press_maybe(
        (value > min + step / 2.0).then(|| {
            let new_val = ((value - step).clamp(min, max) / step).round() * step;
            (on_slider_change)(new_val)
        }),
    )
    .style(stepper_button_style);

    // Plus button
    let plus_btn = button(
        container(phosphor_icon(PhosphorIcon::Plus, 10.0, color::TEXT_SECONDARY))
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .on_press_maybe(
        (value < max - step / 2.0).then(|| {
            let new_val = ((value + step).clamp(min, max) / step).round() * step;
            (on_slider_change)(new_val)
        }),
    )
    .style(stepper_button_style);

    // Current value display (centered in the input area)
    let value_display = container(
        text(format_value(value, step))
            .size(typography::CAPTION)
            .color(color::TEXT_PRIMARY),
    )
    .width(Length::Fixed(48.0))
    .height(Length::Fixed(24.0))
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .style(|_theme: &iced::Theme| iced::widget::container::Style {
        background: Some(iced::Background::Color(color::BG_INSET)),
        border: iced::Border {
            color: color::BORDER_SUBTLE,
            width: 1.0,
            radius: spacing::RADIUS_SM.into(),
        },
        ..Default::default()
    });

    row![
        minus_btn,
        container(slider_elem).width(Length::Fill),
        value_display,
        plus_btn,
    ]
    .spacing(spacing::SM)
    .align_y(Alignment::Center)
    .height(Length::Fixed(spacing::CONTROL_HEIGHT))
    .width(Length::Fill)
    .into()
}

/// Format value based on step precision
fn format_value(value: f32, step: f32) -> String {
    if step >= 1.0 {
        format!("{:.0}", value)
    } else if step >= 0.1 {
        format!("{:.1}", value)
    } else {
        format!("{:.2}", value)
    }
}

/// Stepper button style — subtle circle buttons for +/-
fn stepper_button_style(_theme: &iced::Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
    let base = iced::widget::button::Style {
        background: Some(iced::Background::Color(color::BG_INSET)),
        border: iced::Border {
            color: color::BORDER_SUBTLE,
            width: 1.0,
            radius: 999.0.into(),
        },
        text_color: color::TEXT_SECONDARY,
        ..Default::default()
    };

    match status {
        iced::widget::button::Status::Hovered => iced::widget::button::Style {
            background: Some(iced::Background::Color(color::FILL_HOVER)),
            border: iced::Border {
                color: color::BORDER_STRONG,
                width: 1.0,
                radius: 999.0.into(),
            },
            ..base
        },
        iced::widget::button::Status::Pressed => iced::widget::button::Style {
            background: Some(iced::Background::Color(color::FILL_SELECTED)),
            ..base
        },
        _ => base,
    }
}
