pub use iced::system::{fetch_information, Information as SystemInfomation};

pub fn parse_system_info(system_info: &SystemInfomation) -> String {
    vec![
        system_info.system_name.as_deref().unwrap_or("Unknown"),
        system_info.system_kernel.as_deref().unwrap_or("Unknown"),
        system_info.system_version.as_deref().unwrap_or("Unknown"),
    ]
    .join(" - ")
}
