mod app;

use app::App;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("DanQing Teams")
        .theme(dq_theme::linear_theme())
        .font(dq_theme::INTER)
        .font(dq_components::PHOSPHOR_REGULAR)
        .font(dq_components::PHOSPHOR_BOLD)
        .font(dq_components::PHOSPHOR_FILL)
        .default_font(dq_theme::inter())
        .window(iced::window::Settings {
            size: iced::Size::new(1200.0, 800.0),
            ..Default::default()
        })
        .run()
}
