use std::path::PathBuf;

use iced::{
    widget::{button, text},
    Element, Task,
};

#[derive(Debug, Default)]
pub struct InstaxFramer {
    selected_file: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum InstaxFramerMessage {
    PickImage,
}

impl InstaxFramer {
    pub(crate) fn init() -> InstaxFramer {
        Self {
            ..Default::default()
        }
    }

    pub(crate) fn view(&self) -> Element<'_, InstaxFramerMessage> {
        let Some(selected_file) = &self.selected_file else {
            return button(text("Pick an image file"))
                .on_press(InstaxFramerMessage::PickImage)
                .into();
        };

        text(format!("{}", selected_file.to_string_lossy())).into()
    }

    pub(crate) fn update(&self, _message: InstaxFramerMessage) -> Task<InstaxFramerMessage> {
        todo!()
    }
}
