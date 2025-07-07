use std::path::PathBuf;

pub use iced::system::{fetch_information, Information as SystemInfomation};
use iced::{
    widget::{button, text},
    Element, Task,
};

use crate::counter_app::CounterMessage;

#[derive(Debug, Default)]
pub struct RustFs {
    pub(crate) watching_path: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub enum RustFsEvent {
    PickFolderClicked,
    PickFolderConfirmed(Option<PathBuf>),
}

impl RustFs {
    pub fn view(&'_ self) -> Element<'_, CounterMessage> {
        if let Some(watching_path) = &self.watching_path {
            text(watching_path.to_string_lossy()).into()
        } else {
            button(text("Pick a folder...!"))
                .on_press(CounterMessage::RustFs(RustFsEvent::PickFolderClicked))
                .into()
        }
    }

    pub(crate) fn update(&mut self, fs_event: RustFsEvent) -> Task<CounterMessage> {
        match fs_event {
            RustFsEvent::PickFolderClicked => {
                let files = rfd::FileDialog::new().pick_folder();

                Task::done(CounterMessage::RustFs(RustFsEvent::PickFolderConfirmed(
                    files,
                )))
            }
            RustFsEvent::PickFolderConfirmed(path_buf) => {
                self.watching_path = path_buf;
                Task::none()
            }
        }
    }
}
