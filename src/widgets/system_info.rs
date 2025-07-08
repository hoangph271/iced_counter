pub use iced::system::{fetch_information, Information as SystemInfomation};
use iced::{
    font::{Family, Style, Weight},
    widget::{row, text},
    Element, Font, Task,
};

#[derive(Debug)]
pub struct SystemInfo {
    pub system_info: Option<SystemInfomation>,
}

#[derive(Debug, Clone)]
pub enum SystemInfoMessage {
    SystemInformationLoaded(SystemInfomation),
}

impl SystemInfo {
    pub(crate) fn init() -> SystemInfo {
        SystemInfo { system_info: None }
    }

    pub(crate) fn view(&self) -> Element<'_, SystemInfoMessage> {
        self.system_info
            .as_ref()
            .map_or(text("...").into(), |system_info| {
                let SystemInfomation {
                    system_name,
                    system_kernel,
                    system_version,
                    ..
                } = system_info;

                row![
                    text(system_name.as_deref().unwrap_or("Unknown"))
                        .font(Font {
                            weight: Weight::Bold,
                            style: Style::Italic,
                            family: Family::Monospace,
                            ..Font::default()
                        })
                        .size(16),
                    text(" • "),
                    text(system_kernel.as_deref().unwrap_or("Unknown"))
                        .font(Font::MONOSPACE)
                        .size(16),
                    text(" • "),
                    text(system_version.as_deref().unwrap_or("Unknown"))
                        .font(Font {
                            weight: Weight::Bold,
                            style: Style::Italic,
                            family: Family::Monospace,
                            ..Font::default()
                        })
                        .size(16),
                ]
                .into()
            })
    }

    pub(crate) fn update(
        &mut self,
        system_info: SystemInfomation,
    ) -> iced::Task<SystemInfoMessage> {
        self.system_info = Some(system_info);

        Task::none()
    }

    pub(crate) fn fetch_information() -> Task<SystemInfoMessage> {
        fetch_information().map(SystemInfoMessage::SystemInformationLoaded)
    }
}
