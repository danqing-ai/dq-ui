use iced::{Color, Font, Length};
use iced::widget::{container, text};
use iced::Element;

// ─── Font Loading ────────────────────────────────────────────────────────────

pub const PHOSPHOR_REGULAR: &[u8] = include_bytes!("../assets/Phosphor-Regular.ttf");
pub const PHOSPHOR_BOLD: &[u8] = include_bytes!("../assets/Phosphor-Bold.ttf");
pub const PHOSPHOR_FILL: &[u8] = include_bytes!("../assets/Phosphor-Fill.ttf");

pub fn phosphor_font() -> Font {
    Font::with_name("Phosphor")
}

pub fn phosphor_bold_font() -> Font {
    Font::with_name("Phosphor-Bold")
}

pub fn phosphor_fill_font() -> Font {
    Font::with_name("Phosphor-Fill")
}

// ─── Icon Weight ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconWeight {
    Regular,
    Bold,
    Fill,
}

// ─── Icon Definitions ────────────────────────────────────────────────────────

/// Phosphor Icons — 1200+ professional icons.
/// Use `phosphor_icon()` to render.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhosphorIcon {
    // Navigation / Studio
    Image,
    VideoCamera,
    SpeakerHigh,
    SquaresFour,
    Images,
    Cube,
    Gear,
    List,
    Queue,

    // Document / Content
    Article,
    FileText,
    Pencil,
    Clock,
    ClockClockwise,

    // Actions
    Sparkle,
    Star,
    CaretDown,
    CaretRight,
    ArrowsClockwise,
    Download,
    MagnifyingGlass,
    X,
    Plus,
    Minus,
    Check,
    Trash,
    Copy,

    // Status
    Circle,
    CircleHalf,
    CircleNotch,
    Warning,
    WarningCircle,
    Info,

    // Media
    Play,
    Pause,
    Stop,
    SkipBack,
    SkipForward,

    // Arrows
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowClockwise,

    // Objects
    House,
    User,
    Users,
    Bell,
    MagnifyingGlassPlus,
    Sliders,
    Wrench,
    Palette,
    MagicWand,

    // Upload / File
    Upload,
    FileImage,
    FilePlus,
    Folder,
    FolderOpen,
}

impl PhosphorIcon {
    pub fn codepoint(self) -> char {
        match self {
            // Navigation / Studio
            PhosphorIcon::Image => '\u{E2CA}',
            PhosphorIcon::VideoCamera => '\u{E4DA}',
            PhosphorIcon::SpeakerHigh => '\u{E44A}',
            PhosphorIcon::SquaresFour => '\u{E464}',
            PhosphorIcon::Images => '\u{E836}',
            PhosphorIcon::Cube => '\u{E1DA}',
            PhosphorIcon::Gear => '\u{E270}',
            PhosphorIcon::List => '\u{E2F0}',
            PhosphorIcon::Queue => '\u{E6AC}',

            // Document / Content
            PhosphorIcon::Article => '\u{E0A8}',
            PhosphorIcon::FileText => '\u{E23A}',
            PhosphorIcon::Pencil => '\u{E3AE}',
            PhosphorIcon::Clock => '\u{E19A}',
            PhosphorIcon::ClockClockwise => '\u{E19E}',

            // Actions
            PhosphorIcon::Sparkle => '\u{E6A2}',
            PhosphorIcon::Star => '\u{E46A}',
            PhosphorIcon::CaretDown => '\u{E136}',
            PhosphorIcon::CaretRight => '\u{E13A}',
            PhosphorIcon::ArrowsClockwise => '\u{E094}',
            PhosphorIcon::Download => '\u{E20A}',
            PhosphorIcon::MagnifyingGlass => '\u{E30C}',
            PhosphorIcon::X => '\u{E4F6}',
            PhosphorIcon::Plus => '\u{E3D4}',
            PhosphorIcon::Minus => '\u{E32A}',
            PhosphorIcon::Check => '\u{E182}',
            PhosphorIcon::Trash => '\u{E4A6}',
            PhosphorIcon::Copy => '\u{E1CA}',

            // Status
            PhosphorIcon::Circle => '\u{E15E}',
            PhosphorIcon::CircleHalf => '\u{E162}',
            PhosphorIcon::CircleNotch => '\u{E166}',
            PhosphorIcon::Warning => '\u{E4EC}',
            PhosphorIcon::WarningCircle => '\u{E4F0}',
            PhosphorIcon::Info => '\u{E29A}',

            // Media
            PhosphorIcon::Play => '\u{E3BA}',
            PhosphorIcon::Pause => '\u{E3A6}',
            PhosphorIcon::Stop => '\u{E468}',
            PhosphorIcon::SkipBack => '\u{E440}',
            PhosphorIcon::SkipForward => '\u{E444}',

            // Arrows
            PhosphorIcon::ArrowUp => '\u{E0A4}',
            PhosphorIcon::ArrowDown => '\u{E096}',
            PhosphorIcon::ArrowLeft => '\u{E098}',
            PhosphorIcon::ArrowRight => '\u{E0A0}',
            PhosphorIcon::ArrowClockwise => '\u{E094}',

            // Objects
            PhosphorIcon::House => '\u{E28A}',
            PhosphorIcon::User => '\u{E4D2}',
            PhosphorIcon::Users => '\u{E4D6}',
            PhosphorIcon::Bell => '\u{E0C2}',
            PhosphorIcon::MagnifyingGlassPlus => '\u{E310}',
            PhosphorIcon::Sliders => '\u{E456}',
            PhosphorIcon::Wrench => '\u{E52E}',
            PhosphorIcon::Palette => '\u{E39E}',
            PhosphorIcon::MagicWand => '\u{E2EE}',

            // Upload / File
            PhosphorIcon::Upload => '\u{E4CE}',
            PhosphorIcon::FileImage => '\u{E22E}',
            PhosphorIcon::FilePlus => '\u{E236}',
            PhosphorIcon::Folder => '\u{E258}',
            PhosphorIcon::FolderOpen => '\u{E25C}',
        }
    }

    pub fn weight(self) -> IconWeight {
        match self {
            // Fill weight for some icons to make them more visible
            PhosphorIcon::Star => IconWeight::Fill,
            PhosphorIcon::Circle => IconWeight::Fill,
            PhosphorIcon::CircleHalf => IconWeight::Fill,
            PhosphorIcon::CircleNotch => IconWeight::Fill,
            PhosphorIcon::Warning => IconWeight::Fill,
            PhosphorIcon::WarningCircle => IconWeight::Fill,
            PhosphorIcon::Info => IconWeight::Fill,
            _ => IconWeight::Regular,
        }
    }
}

// ─── Render Functions ────────────────────────────────────────────────────────

/// Render a Phosphor icon as an Iced text element.
pub fn phosphor_icon<'a, Message: 'a>(
    icon: PhosphorIcon,
    size: f32,
    color: Color,
) -> Element<'a, Message> {
    let font = match icon.weight() {
        IconWeight::Regular => phosphor_font(),
        IconWeight::Bold => phosphor_bold_font(),
        IconWeight::Fill => phosphor_fill_font(),
    };

    text(icon.codepoint())
        .font(font)
        .size(size)
        .color(color)
        .into()
}

/// Render a Phosphor icon wrapped in a fixed-size container (centered).
pub fn phosphor_icon_box<'a, Message: 'a>(
    icon: PhosphorIcon,
    size: f32,
    color: Color,
    box_size: f32,
) -> Element<'a, Message> {
    container(phosphor_icon(icon, size, color))
        .width(Length::Fixed(box_size))
        .height(Length::Fixed(box_size))
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .into()
}

// ─── Convenience Aliases for Legacy Migration ────────────────────────────────
// Note: brand_logo, image_placeholder_icon, sparkle_icon, list_icon,
// chevron_icon, refresh_icon, thumb_icon are defined in icons.rs to maintain
// backward compatibility. Use phosphor_icon() directly for new code.
