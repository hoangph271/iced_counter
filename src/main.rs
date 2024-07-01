use iced::{Size, Theme};

mod counter_app;
mod counter_themes;

use counter_app::{CounterApp, CounterMessage};
use counter_themes::theme_from;

mod system_info;
use system_info::fetch_information;

fn main() -> iced::Result {
    iced::application(
        "iced_counter by @sneu",
        CounterApp::update,
        CounterApp::view,
    )
    .theme(|state| theme_from(&state.theme_name, &state.dark_theme.unwrap_or_default()))
    .window_size(Size {
        width: 512.0,
        height: 240.0,
    })
    .load(|| fetch_information().map(CounterMessage::SystemInfoLoaded))
    .subscription(CounterApp::subscription)
    .run_with(|| CounterApp {
        value: Default::default(),
        allow_negative: true,
        dark_theme: Some(Theme::default() == Theme::Dark),
        theme_name: counter_themes::GRUVBOX.to_owned(),
        system_info: None,
    })
}
