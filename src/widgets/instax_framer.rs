use iced::{widget::text, Element};

#[derive(Debug)]
pub struct InstaxFramer {}

impl InstaxFramer {
    pub(crate) fn init() -> InstaxFramer {
        Self {}
    }

    pub(crate) fn view(&self) -> Element<'_, InstaxFramerMessage> {
        text("...").into()
    }
}

#[derive(Debug, Clone)]
pub enum InstaxFramerMessage {}
