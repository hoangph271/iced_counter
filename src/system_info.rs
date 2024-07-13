pub use iced::system::{fetch_information, Information as SystemInfomation};
use iced::{
    widget::{row, text},
    Element,
};

use crate::counter_app::CounterMessage;

pub fn system_info_view(system_info: &SystemInfomation) -> Element<CounterMessage> {
    let SystemInfomation {
        system_name,
        system_kernel,
        system_version,
        ..
    } = system_info;

    row![
        text(system_name.as_deref().unwrap_or("Unknown")),
        text("-"),
        text(system_kernel.as_deref().unwrap_or("Unknown")),
        text("-"),
        text(system_version.as_deref().unwrap_or("Unknown")),
    ]
    .into()
}
