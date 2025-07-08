#[cfg(feature = "system_info")]
use crate::omni_app::OmniAppMessage;
use iced::Size;
#[cfg(not(feature = "system_info"))]
use iced::Task;

mod omni_app;
mod widgets;

use omni_app::OmniApp;
use widgets::*;

fn main() -> iced::Result {
    iced::application(
        || {
            (
                OmniApp::init(),
                #[cfg(feature = "system_info")]
                system_info::fetch_information().map(OmniAppMessage::SystemInfoLoaded),
                #[cfg(not(feature = "system_info"))]
                Task::none(),
            )
        },
        OmniApp::update,
        OmniApp::view,
    )
    .title("iced_counter by @sneu")
    .theme(counter_themes::theme_from_state)
    .window_size(Size {
        width: 544.0,
        height: 288.0,
    })
    .subscription(OmniApp::subscription)
    .run()
}
