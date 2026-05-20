use iced::Size;

mod constants;
mod features;
mod omni_app;

use constants::APP_NAME_WITH_AUTHOR;
use omni_app::OmniApp;

fn main() -> iced::Result {
    let app = iced::application(
        move || {
            let mut omni_app = OmniApp::init();
            let start_up_tasks = omni_app.start_up_tasks();

            (omni_app, start_up_tasks)
        },
        OmniApp::update,
        OmniApp::view,
    )
    .title(APP_NAME_WITH_AUTHOR)
    .theme(OmniApp::theme)
    .window_size(Size {
        width: 544.0,
        height: 288.0,
    })
    .exit_on_close_request(false)
    .subscription(OmniApp::subscription);

    app.run()
}
