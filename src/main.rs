use iced::Size;

mod counter_app;
mod counter_themes;

use counter_app::{CounterApp, CounterMessage};
use counter_themes::{theme_from_state, ThemeMode};

mod system_info;
use system_info::fetch_information;

fn main() -> iced::Result {
    iced::application(
        "iced_counter by @sneu",
        CounterApp::update,
        CounterApp::view,
    )
    .theme(theme_from_state)
    .window_size(Size {
        width: 512.0,
        height: 320.0,
    })
    .load(|| fetch_information().map(CounterMessage::SystemInfoLoaded))
    .subscription(CounterApp::subscription)
    .run_with(|| CounterApp {
        value: Default::default(),
        allow_negative: true,
        auto_increment_enabled: false,
        application_theme_mode: ThemeMode::SystemDefault,
        system_theme_mode: counter_themes::get_system_theme_mode(),
        theme_name: counter_themes::GRUVBOX.to_owned(),
        system_info: None,
    })
}
