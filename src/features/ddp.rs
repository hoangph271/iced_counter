use iced::{widget, Element, Task};

#[derive(Debug, Default)]
pub(crate) struct Ddp {}

#[derive(Debug, Clone)]
pub(crate) enum DdpMessage {}

impl Ddp {
    pub(crate) fn init() -> Self {
        Self {}
    }

    pub(crate) fn view(&self) -> Element<'_, DdpMessage> {
        widget::text("DDP").into()
    }

    pub(crate) fn update(&self, _message: DdpMessage) -> Task<DdpMessage> {
        Task::none()
    }
}
