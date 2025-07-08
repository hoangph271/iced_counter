use iced::Size;

mod counter;
mod counter_themes;
mod omni_app;

use counter_themes::theme_from_state;
use omni_app::{OmniApp, OmniAppMessage};

mod system_info;
use system_info::fetch_information;

fn main() -> iced::Result {
    iced::application(
        || {
            (
                OmniApp::init(),
                fetch_information().map(OmniAppMessage::SystemInfoLoaded),
            )
        },
        OmniApp::update,
        OmniApp::view,
    )
    .title("iced_counter by @sneu")
    .theme(theme_from_state)
    .window_size(Size {
        width: 544.0,
        height: 288.0,
    })
    .subscription(OmniApp::subscription)
    .run()
}
