use iced::Font;

pub const INTER: &[u8] = include_bytes!("../assets/Inter-Regular.ttf");

pub fn inter() -> Font {
    Font::with_name("Inter")
}
