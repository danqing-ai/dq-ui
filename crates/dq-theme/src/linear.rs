use dq_tokens::{generate, semantic::SemanticPalette, ThemeInput};
use iced::theme::{Custom, Palette, Theme};
use iced::widget::container;
use iced::Color;
use std::sync::Arc;

/// Build an Iced theme from a semantic palette.
pub fn theme_from_palette(palette: SemanticPalette, name: impl Into<String>) -> Theme {
    let iced_palette = Palette {
        background: palette.bg_page,
        text: palette.text_primary,
        primary: palette.accent,
        success: palette.success,
        danger: palette.danger,
        warning: palette.warning,
    };

    Theme::Custom(Arc::new(Custom::new(name.into(), iced_palette)))
}

/// Linear dark theme (default).
pub fn linear_theme() -> Theme {
    theme_from_palette(SemanticPalette::dark(), "Linear Dark")
}

/// Linear light theme.
pub fn linear_light_theme() -> Theme {
    theme_from_palette(SemanticPalette::light(), "Linear Light")
}

/// Custom Linear-style theme from base + accent + contrast knobs.
pub fn linear_theme_custom(input: ThemeInput) -> Theme {
    let palette = generate(input);
    let name = if input.is_dark {
        "Linear Dark (custom)"
    } else {
        "Linear Light (custom)"
    };
    theme_from_palette(palette, name)
}

fn hairline_bottom(separator: Color) -> container::Style {
    container::Style {
        border: iced::Border {
            color: separator,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: iced::Shadow {
            color: separator,
            offset: iced::Vector::new(0.0, 1.0),
            blur_radius: 0.0,
        },
        ..Default::default()
    }
}

/// Sidebar shell — deepest chrome layer (inverted-L vertical arm).
pub fn sidebar_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(dq_tokens::color::BG_BASE)),
        ..hairline_bottom(dq_tokens::color::SEPARATOR)
    }
}

/// Main content area background.
pub fn page_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(dq_tokens::color::BG_PAGE)),
        ..Default::default()
    }
}



/// Top navigation bar for Studio shell.
pub fn studio_nav_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(dq_tokens::color::BG_BASE)),
        ..hairline_bottom(dq_tokens::color::SEPARATOR)
    }
}

/// Mode tabs row — clean, no hairline (pill tabs float on page background).
pub fn mode_tabs_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(dq_tokens::color::BG_PAGE)),
        ..Default::default()
    }
}

/// Hairline between section header and body.
pub fn section_divider(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(dq_tokens::color::SEPARATOR)),
        ..Default::default()
    }
}

/// Form section card — elevated panel on page background.
/// Linear style: depth comes from subtle luminance lift + whisper shadow.
pub fn panel_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(dq_tokens::color::BG_PANEL)),
        border: iced::Border {
            color: dq_tokens::color::BORDER_SUBTLE,
            width: 1.0,
            radius: dq_tokens::spacing::RADIUS_LG.into(),
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.25),
            offset: iced::Vector::new(0.0, 1.0),
            blur_radius: 4.0,
        },
        ..Default::default()
    }
}

/// Top navigation bar with bottom hairline.
pub fn topnav_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(dq_tokens::color::BG_PAGE)),
        ..hairline_bottom(dq_tokens::color::SEPARATOR)
    }
}

/// Vertical divider between columns.
pub fn vertical_divider(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(dq_tokens::color::SEPARATOR)),
        ..Default::default()
    }
}

/// Secondary surface for containers (cards).
pub fn elevated_container(_theme: &Theme) -> container::Style {
    panel_container(_theme)
}

/// Inset panel — recessed control well (inputs, preview wells).
/// Linear style: barely-there border, deeper background creates the recess.
pub fn inset_panel(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(dq_tokens::color::BG_INSET)),
        border: iced::Border {
            color: dq_tokens::color::BORDER_SUBTLE,
            width: 1.0,
            radius: dq_tokens::spacing::RADIUS_MD.into(),
        },
        ..Default::default()
    }
}

/// Brand row bottom separator in sidebar.
pub fn sidebar_header_divider(_theme: &Theme) -> container::Style {
    container::Style {
        ..hairline_bottom(dq_tokens::color::SEPARATOR)
    }
}

pub fn inset_border(color: Color) -> iced::Border {
    iced::Border {
        color,
        width: 1.0,
        radius: dq_tokens::spacing::RADIUS_MD.into(),
    }
}

/// @deprecated use sidebar_container
pub fn surface_container(theme: &Theme) -> container::Style {
    sidebar_container(theme)
}

/// Subtle scrollbar — thin, dark, minimal (Linear/Figma style).
pub fn subtle_scrollbar(_theme: &Theme, _status: iced::widget::scrollable::Status) -> iced::widget::scrollable::Style {
    let scroller_bg = iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.12));
    
    iced::widget::scrollable::Style {
        container: container::Style {
            background: None,
            ..Default::default()
        },
        vertical_rail: iced::widget::scrollable::Rail {
            background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.0))),
            border: iced::Border::default(),
            scroller: iced::widget::scrollable::Scroller {
                background: scroller_bg,
                border: iced::Border::default(),
            },
        },
        horizontal_rail: iced::widget::scrollable::Rail {
            background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.0))),
            border: iced::Border::default(),
            scroller: iced::widget::scrollable::Scroller {
                background: scroller_bg,
                border: iced::Border::default(),
            },
        },
        gap: None,
        auto_scroll: iced::widget::scrollable::AutoScroll {
            background: iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.0)),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
            icon: Color::from_rgba(1.0, 1.0, 1.0, 0.0),
        },
    }
}
