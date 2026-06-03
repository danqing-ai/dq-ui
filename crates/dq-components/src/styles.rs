use dq_tokens::color;
use iced::widget::button;
use iced::{Background, Border, Color, Theme};

fn radius_md() -> iced::border::Radius {
    dq_tokens::spacing::radius_control().into()
}

/// Primary CTA — flat accent, no glow (Linear keeps chrome neutral).
pub fn primary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(color::accent())),
        text_color: Color::WHITE,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: radius_md(),
        },
        shadow: iced::Shadow::default(),
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(color::accent_hover())),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(color::accent_muted())),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgba(
                color::accent().r,
                color::accent().g,
                color::accent().b,
                0.40,
            ))),
            text_color: Color::from_rgba(1.0, 1.0, 1.0, 0.80),
            ..base
        },
        _ => base,
    }
}

/// Secondary — elevated surface + hairline border.
pub fn secondary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(color::bg_surface())),
        text_color: color::text_primary(),
        border: Border {
            color: color::border_subtle(),
            width: 1.0,
            radius: radius_md(),
        },
        shadow: iced::Shadow::default(),
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
                background: Some(Background::Color(color::bg_elevated())),
            border: Border {
                color: color::border_strong(),
                ..base.border
            },
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(color::bg_elevated())),
            ..base
        },
        button::Status::Disabled => button::Style {
            text_color: color::text_tertiary(),
            ..base
        },
        _ => base,
    }
}

/// Ghost — transparent until hover fill.
pub fn ghost_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: None,
        text_color: color::text_secondary(),
        border: Border::default(),
        shadow: iced::Shadow::default(),
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(color::fill_hover())),
            text_color: color::text_primary(),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(color::fill_active())),
            text_color: color::text_primary(),
            ..base
        },
        _ => base,
    }
}

/// Sidebar / list nav item.
pub fn nav_item_button(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let base = button::Style {
            background: if active {
                Some(Background::Color(color::fill_selected()))
            } else {
                None
            },
            text_color: if active {
                color::text_primary()
            } else {
                color::text_secondary()
            },
            border: Border::default(),
            ..Default::default()
        };
        if !active {
            match status {
                button::Status::Hovered => button::Style {
                    background: Some(Background::Color(color::fill_hover())),
                    text_color: color::text_primary(),
                    ..base
                },
                _ => base,
            }
        } else {
            base
        }
    }
}

/// Top studio nav — subtle fill capsule, no border noise.
pub fn studio_nav_pill(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let text_color = match status {
            button::Status::Hovered if !active => color::text_primary(),
            _ if active => color::text_primary(),
            _ => color::text_secondary(),
        };

        let background = if active {
            Some(Background::Color(color::fill_selected()))
        } else {
            match status {
                button::Status::Hovered => Some(Background::Color(color::fill_hover())),
                _ => None,
            }
        };

        button::Style {
            background,
            text_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: radius_md(),
            },
            shadow: iced::Shadow::default(),
            ..Default::default()
        }
    }
}

/// Secondary mode tabs — transparent, underline only.
pub fn mode_tab_button(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let text_color = match status {
            button::Status::Hovered if !active => color::text_primary(),
            _ if active => color::text_primary(),
            _ => color::text_secondary(),
        };

        button::Style {
            background: None,
            text_color,
            border: Border::default(),
            shadow: iced::Shadow::default(),
            ..Default::default()
        }
    }
}

pub fn studio_nav_button(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    studio_nav_pill(active)
}

pub fn mode_tab_pill(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    studio_nav_pill(active)
}

/// Inset control — matches text_input / pick_list chrome.
pub fn control_inset_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(color::bg_inset())),
        text_color: color::text_primary(),
        border: Border {
            color: color::border_subtle(),
            width: 1.0,
            radius: radius_md(),
        },
        shadow: iced::Shadow::default(),
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
            border: Border {
                color: color::border_strong(),
                width: 1.0,
                radius: radius_md(),
            },
            background: Some(Background::Color(color::bg_elevated())),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(color::bg_inset())),
            ..base
        },
        button::Status::Disabled => button::Style {
            text_color: color::text_tertiary(),
            ..base
        },
        _ => base,
    }
}

pub fn text_editor_style(
    _theme: &Theme,
    status: iced::widget::text_editor::Status,
) -> iced::widget::text_editor::Style {
    let base = iced::widget::text_editor::Style {
        background: Background::Color(color::bg_inset()),
        border: Border {
            color: color::border_subtle(),
            width: 1.0,
            radius: radius_md(),
        },
        placeholder: color::text_quaternary(),
        value: color::text_primary(),
        selection: color::accent(),
    };
    match status {
        iced::widget::text_editor::Status::Focused { .. } => iced::widget::text_editor::Style {
            border: Border {
                color: color::accent(),
                width: 1.0,
                radius: radius_md(),
            },
            background: Background::Color(color::bg_elevated()),
            ..base
        },
        iced::widget::text_editor::Status::Hovered => iced::widget::text_editor::Style {
            border: Border {
                color: color::border_strong(),
                width: 1.0,
                radius: radius_md(),
            },
            ..base
        },
        _ => base,
    }
}

pub fn text_input_style(_theme: &Theme, status: iced::widget::text_input::Status) -> iced::widget::text_input::Style {
    let base = iced::widget::text_input::Style {
        background: Background::Color(color::bg_inset()),
        border: Border {
            color: color::border_subtle(),
            width: 1.0,
            radius: radius_md(),
        },
        icon: color::icon_tertiary(),
        placeholder: color::text_quaternary(),
        value: color::text_primary(),
        selection: color::accent(),
    };
    match status {
        iced::widget::text_input::Status::Focused { .. } => iced::widget::text_input::Style {
            border: Border {
                color: color::accent(),
                width: 1.0,
                radius: radius_md(),
            },
            background: Background::Color(color::bg_elevated()),
            ..base
        },
        iced::widget::text_input::Status::Hovered => iced::widget::text_input::Style {
            border: Border {
                color: color::border_strong(),
                width: 1.0,
                radius: radius_md(),
            },
            ..base
        },
        _ => base,
    }
}

pub fn pick_list_style(_theme: &Theme, status: iced::widget::pick_list::Status) -> iced::widget::pick_list::Style {
    let base = iced::widget::pick_list::Style {
        background: Background::Color(color::bg_inset()),
        border: Border {
            color: color::border_subtle(),
            width: 1.0,
            radius: radius_md(),
        },
        text_color: color::text_primary(),
        placeholder_color: color::text_quaternary(),
        handle_color: color::icon_tertiary(),
    };
    match status {
        iced::widget::pick_list::Status::Hovered => iced::widget::pick_list::Style {
            border: Border {
                color: color::border_strong(),
                ..base.border
            },
            background: Background::Color(color::bg_elevated()),
            ..base
        },
        iced::widget::pick_list::Status::Opened { .. } => iced::widget::pick_list::Style {
            border: Border {
                color: color::accent(),
                width: 1.0,
                ..base.border
            },
            background: Background::Color(color::bg_elevated()),
            ..base
        },
        _ => base,
    }
}

pub fn slider_style(_theme: &Theme, status: iced::widget::slider::Status) -> iced::widget::slider::Style {
    let handle_bg = match status {
        iced::widget::slider::Status::Hovered => Background::Color(Color::WHITE),
        iced::widget::slider::Status::Dragged => Background::Color(color::accent()),
        _ => Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.90)),
    };

    iced::widget::slider::Style {
        rail: iced::widget::slider::Rail {
            backgrounds: (
                Background::Color(color::accent()),
                Background::Color(color::bg_surface()),
            ),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 999.0.into(),
            },
            width: 4.0,
        },
        handle: iced::widget::slider::Handle {
            background: handle_bg,
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
            shape: iced::widget::slider::HandleShape::Circle { radius: 6.0 },
        },
    }
}

/// Segmented / view tabs — selected capsule with hairline.
pub fn tab_button(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let base = button::Style {
            background: if active {
                Some(Background::Color(color::fill_selected()))
            } else {
                None
            },
            text_color: if active {
                color::text_primary()
            } else {
                color::text_secondary()
            },
            border: Border {
                color: if active {
                    color::border_subtle()
                } else {
                    Color::TRANSPARENT
                },
                width: 1.0,
                radius: (dq_tokens::spacing::radius_control() - 1.0).into(),
            },
            shadow: iced::Shadow::default(),
            ..Default::default()
        };

        if !active {
            match status {
                button::Status::Hovered => button::Style {
                    text_color: color::text_primary(),
                    background: Some(Background::Color(color::fill_hover())),
                    ..base
                },
                _ => base,
            }
        } else {
            base
        }
    }
}
