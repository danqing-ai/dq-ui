use dq_tokens::color;
use iced::widget::button;
use iced::{Background, Border, Color, Theme};

fn radius_md() -> iced::border::Radius {
    dq_tokens::spacing::RADIUS_MD.into()
}

/// Primary CTA — flat accent, no glow (Linear keeps chrome neutral).
pub fn primary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(color::ACCENT)),
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
            background: Some(Background::Color(color::ACCENT_HOVER)),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(color::ACCENT_MUTED)),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgba(
                color::ACCENT.r,
                color::ACCENT.g,
                color::ACCENT.b,
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
        background: Some(Background::Color(color::BG_SURFACE)),
        text_color: color::TEXT_PRIMARY,
        border: Border {
            color: color::BORDER_SUBTLE,
            width: 1.0,
            radius: radius_md(),
        },
        shadow: iced::Shadow::default(),
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
                background: Some(Background::Color(color::BG_ELEVATED)),
            border: Border {
                color: color::BORDER_STRONG,
                ..base.border
            },
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(color::BG_ELEVATED)),
            ..base
        },
        button::Status::Disabled => button::Style {
            text_color: color::TEXT_TERTIARY,
            ..base
        },
        _ => base,
    }
}

/// Ghost — transparent until hover fill.
pub fn ghost_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: None,
        text_color: color::TEXT_SECONDARY,
        border: Border::default(),
        shadow: iced::Shadow::default(),
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(color::FILL_HOVER)),
            text_color: color::TEXT_PRIMARY,
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(color::FILL_ACTIVE)),
            text_color: color::TEXT_PRIMARY,
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
                Some(Background::Color(color::FILL_SELECTED))
            } else {
                None
            },
            text_color: if active {
                color::TEXT_PRIMARY
            } else {
                color::TEXT_SECONDARY
            },
            border: Border::default(),
            ..Default::default()
        };
        if !active {
            match status {
                button::Status::Hovered => button::Style {
                    background: Some(Background::Color(color::FILL_HOVER)),
                    text_color: color::TEXT_PRIMARY,
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
            button::Status::Hovered if !active => color::TEXT_PRIMARY,
            _ if active => color::TEXT_PRIMARY,
            _ => color::TEXT_SECONDARY,
        };

        let background = if active {
            Some(Background::Color(color::FILL_SELECTED))
        } else {
            match status {
                button::Status::Hovered => Some(Background::Color(color::FILL_HOVER)),
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
            button::Status::Hovered if !active => color::TEXT_PRIMARY,
            _ if active => color::TEXT_PRIMARY,
            _ => color::TEXT_SECONDARY,
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
        background: Some(Background::Color(color::BG_INSET)),
        text_color: color::TEXT_PRIMARY,
        border: Border {
            color: color::BORDER_SUBTLE,
            width: 1.0,
            radius: radius_md(),
        },
        shadow: iced::Shadow::default(),
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
            border: Border {
                color: color::BORDER_STRONG,
                width: 1.0,
                radius: radius_md(),
            },
            background: Some(Background::Color(color::BG_ELEVATED)),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(color::BG_INSET)),
            ..base
        },
        button::Status::Disabled => button::Style {
            text_color: color::TEXT_TERTIARY,
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
        background: Background::Color(color::BG_INSET),
        border: Border {
            color: color::BORDER_SUBTLE,
            width: 1.0,
            radius: radius_md(),
        },
        placeholder: color::TEXT_QUATERNARY,
        value: color::TEXT_PRIMARY,
        selection: color::ACCENT,
    };
    match status {
        iced::widget::text_editor::Status::Focused { .. } => iced::widget::text_editor::Style {
            border: Border {
                color: color::ACCENT,
                width: 1.0,
                radius: radius_md(),
            },
            background: Background::Color(color::BG_ELEVATED),
            ..base
        },
        iced::widget::text_editor::Status::Hovered => iced::widget::text_editor::Style {
            border: Border {
                color: color::BORDER_STRONG,
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
        background: Background::Color(color::BG_INSET),
        border: Border {
            color: color::BORDER_SUBTLE,
            width: 1.0,
            radius: radius_md(),
        },
        icon: color::ICON_TERTIARY,
        placeholder: color::TEXT_QUATERNARY,
        value: color::TEXT_PRIMARY,
        selection: color::ACCENT,
    };
    match status {
        iced::widget::text_input::Status::Focused { .. } => iced::widget::text_input::Style {
            border: Border {
                color: color::ACCENT,
                width: 1.0,
                radius: radius_md(),
            },
            background: Background::Color(color::BG_ELEVATED),
            ..base
        },
        iced::widget::text_input::Status::Hovered => iced::widget::text_input::Style {
            border: Border {
                color: color::BORDER_STRONG,
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
        background: Background::Color(color::BG_INSET),
        border: Border {
            color: color::BORDER_SUBTLE,
            width: 1.0,
            radius: radius_md(),
        },
        text_color: color::TEXT_PRIMARY,
        placeholder_color: color::TEXT_QUATERNARY,
        handle_color: color::ICON_TERTIARY,
    };
    match status {
        iced::widget::pick_list::Status::Hovered => iced::widget::pick_list::Style {
            border: Border {
                color: color::BORDER_STRONG,
                ..base.border
            },
            background: Background::Color(color::BG_ELEVATED),
            ..base
        },
        iced::widget::pick_list::Status::Opened { .. } => iced::widget::pick_list::Style {
            border: Border {
                color: color::ACCENT,
                width: 1.0,
                ..base.border
            },
            background: Background::Color(color::BG_ELEVATED),
            ..base
        },
        _ => base,
    }
}

pub fn slider_style(_theme: &Theme, status: iced::widget::slider::Status) -> iced::widget::slider::Style {
    let handle_bg = match status {
        iced::widget::slider::Status::Hovered => Background::Color(Color::WHITE),
        iced::widget::slider::Status::Dragged => Background::Color(color::ACCENT),
        _ => Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.90)),
    };

    iced::widget::slider::Style {
        rail: iced::widget::slider::Rail {
            backgrounds: (
                Background::Color(color::ACCENT),
                Background::Color(color::BG_SURFACE),
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
                Some(Background::Color(color::FILL_SELECTED))
            } else {
                None
            },
            text_color: if active {
                color::TEXT_PRIMARY
            } else {
                color::TEXT_SECONDARY
            },
            border: Border {
                color: if active {
                    color::BORDER_SUBTLE
                } else {
                    Color::TRANSPARENT
                },
                width: 1.0,
                radius: (dq_tokens::spacing::RADIUS_MD - 1.0).into(),
            },
            shadow: iced::Shadow::default(),
            ..Default::default()
        };

        if !active {
            match status {
                button::Status::Hovered => button::Style {
                    text_color: color::TEXT_PRIMARY,
                    background: Some(Background::Color(color::FILL_HOVER)),
                    ..base
                },
                _ => base,
            }
        } else {
            base
        }
    }
}
