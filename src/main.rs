use iced::Size;

mod omni_app;
mod widgets;

use omni_app::OmniApp;
use widgets::*;

fn main() -> iced::Result {
    iced::application(
        || (OmniApp::init(), OmniApp::start_up_tasks()),
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
