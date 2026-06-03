//! PrefPane / PrefRow — form layout components for settings and parameter forms.
//! Matches web UI's DqPrefPane + DqPrefRow pattern.

use dq_tokens::{color, spacing, typography};
use iced::widget::{column, container, row, text, Space};
use iced::{Alignment, Element, Length};

/// PrefPane — groups pref rows into a cohesive form section.
pub fn pref_pane<'a, Message: Clone + 'a>(
    children: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message> {
    let mut col = column![].spacing(spacing::MD).width(Length::Fill);
    for child in children {
        col = col.push(child);
    }
    col.into()
}

/// PrefRow — form row with fixed-width label on the left, control on the right.
/// Optional `hint` text shown below the control.
/// `stacked` variant puts label above control (for textareas, uploads).
pub fn pref_row<'a, Message: Clone + 'a>(
    label: &'a str,
    control: Element<'a, Message>,
    hint: Option<&'a str>,
    stacked: bool,
) -> Element<'a, Message> {
    let label_el = text(label).size(typography::LABEL).color(color::text_secondary());

    let body: Element<'a, Message> = if stacked {
        let hint_el: Element<'a, Message> = if let Some(h) = hint {
            text(h).size(typography::MINI).color(color::text_quaternary()).into()
        } else {
            Space::new().height(0).into()
        };
        column![
            label_el,
            control,
            hint_el,
        ]
        .spacing(spacing::XS)
        .width(Length::Fill)
        .into()
    } else {
        let hint_el: Element<'a, Message> = if let Some(h) = hint {
            text(h).size(typography::MINI).color(color::text_quaternary()).into()
        } else {
            Space::new().width(0).into()
        };
        row![
            container(label_el)
                .width(Length::Fixed(100.0))
                .align_y(Alignment::Center),
            control,
            hint_el,
        ]
        .spacing(spacing::SM)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .into()
    };

    body
}

/// Shortcut for inline pref row (label + control side by side).
pub fn pref_row_inline<'a, Message: Clone + 'a>(
    label: &'a str,
    control: Element<'a, Message>,
) -> Element<'a, Message> {
    pref_row(label, control, None, false)
}

/// Shortcut for stacked pref row (label above control).
pub fn pref_row_stacked<'a, Message: Clone + 'a>(
    label: &'a str,
    control: Element<'a, Message>,
) -> Element<'a, Message> {
    pref_row(label, control, None, true)
}
