use iced::Size;

mod counter_app;
mod counter_themes;

use counter_app::{CounterApp, CounterMessage};
use counter_themes::theme_from_state;

mod rust_fs;
use rust_fs::fetch_information;

fn main() -> iced::Result {
    iced::application(
        || {
            (
                CounterApp::new(),
                fetch_information().map(CounterMessage::SystemInfoLoaded),
            )
        },
        CounterApp::update,
        CounterApp::view,
    )
    .title("iced_counter by @sneu")
    .theme(theme_from_state)
    .window_size(Size {
        width: 544.0,
        height: 288.0,
    })
    .subscription(CounterApp::subscription)
    .run()
}
