use iced::Size;

mod features;
mod omni_app;

use omni_app::OmniApp;

fn main() -> iced::Result {
    let app = iced::application(
        || {
            let omni_app = OmniApp::init();
            let start_up_tasks = omni_app.start_up_tasks();

            (omni_app, start_up_tasks)
        },
        OmniApp::update,
        OmniApp::view,
    )
    .title("omni_app by @sneu")
    .theme(OmniApp::theme)
    .window_size(Size {
        width: 544.0,
        height: 288.0,
    })
    .subscription(OmniApp::subscription);

    app.run()
}
