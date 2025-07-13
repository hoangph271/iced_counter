use iced::{
    alignment::{Horizontal, Vertical},
    widget::{column, container, vertical_space},
    Alignment, Element, Length, Subscription, Task,
};

#[cfg(feature = "omni_themes")]
use crate::features::omni_themes::{OmniThemes, OmniThemesMessage};
#[cfg(feature = "system_info")]
use crate::features::system_info::{SystemInfo, SystemInfoMessage};

#[cfg(feature = "counter")]
use crate::features::counter::{Counter, CounterMessage};

#[cfg(feature = "instax_framer")]
use crate::features::instax_framer::{InstaxFramer, InstaxFramerMessage};

#[cfg(feature = "ddp")]
use crate::features::ddp::{Ddp, DdpMessage};

#[derive(Debug)]
pub(super) struct OmniApp {
    #[cfg(feature = "omni_themes")]
    pub omni_themes: OmniThemes,
    #[cfg(feature = "counter")]
    counter: Counter,
    #[cfg(feature = "system_info")]
    pub system_info: SystemInfo,
    #[cfg(feature = "instax_framer")]
    pub instax_framer: InstaxFramer,
    #[cfg(feature = "ddp")]
    pub ddp: Ddp,
}

#[derive(Clone, Debug)]
pub enum OmniAppMessage {
    #[cfg(feature = "counter")]
    CounterEvent(CounterMessage),
    #[cfg(feature = "system_info")]
    SystemInfo(SystemInfoMessage),
    #[cfg(feature = "instax_framer")]
    InstaxFramer(InstaxFramerMessage),
    #[cfg(feature = "ddp")]
    Ddp(DdpMessage),
    #[cfg(feature = "omni_themes")]
    OmniThemes(OmniThemesMessage),
}

impl OmniApp {
    pub fn init() -> Self {
        Self {
            #[cfg(feature = "omni_themes")]
            omni_themes: OmniThemes::init(),
            #[cfg(feature = "counter")]
            counter: Counter::init(),
            #[cfg(feature = "system_info")]
            system_info: SystemInfo::init(),
            #[cfg(feature = "instax_framer")]
            instax_framer: InstaxFramer::init(),
            #[cfg(feature = "ddp")]
            ddp: Ddp::init(),
        }
    }

    pub fn view(&'_ self) -> Element<'_, OmniAppMessage> {
        container(
            column![vertical_space().height(4)]
                .push_maybe(
                    #[cfg(feature = "omni_themes")]
                    Some(self.omni_themes.view().map(OmniAppMessage::OmniThemes)),
                    #[cfg(not(feature = "omni_themes"))]
                    None::<Element<'_, OmniAppMessage>>,
                )
                .push_maybe(
                    #[cfg(feature = "counter")]
                    Some(self.counter.view().map(OmniAppMessage::CounterEvent)),
                    #[cfg(not(feature = "counter"))]
                    None::<Element<'_, OmniAppMessage>>,
                )
                .push_maybe(
                    #[cfg(feature = "system_info")]
                    Some(self.system_info.view().map(OmniAppMessage::SystemInfo)),
                    #[cfg(not(feature = "system_info"))]
                    None::<Element<'_, OmniAppMessage>>,
                )
                .push_maybe(
                    #[cfg(feature = "instax_framer")]
                    Some(self.instax_framer.view().map(OmniAppMessage::InstaxFramer)),
                    #[cfg(not(feature = "instax_framer"))]
                    None::<Element<'_, OmniAppMessage>>,
                )
                .push_maybe(
                    #[cfg(feature = "ddp")]
                    Some(self.ddp.view().map(OmniAppMessage::Ddp)),
                    #[cfg(not(feature = "ddp"))]
                    None::<Element<'_, OmniAppMessage>>,
                )
                .align_x(Alignment::Center)
                .spacing(12)
                .height(Length::Fill),
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .into()
    }

    pub fn update(&mut self, message: OmniAppMessage) -> Task<OmniAppMessage> {
        match message {
            #[cfg(feature = "counter")]
            OmniAppMessage::CounterEvent(counter_event) => self
                .counter
                .update(counter_event)
                .map(OmniAppMessage::CounterEvent),
            #[cfg(feature = "omni_themes")]
            OmniAppMessage::OmniThemes(message) => self
                .omni_themes
                .update(message)
                .map(OmniAppMessage::OmniThemes),
            #[cfg(feature = "system_info")]
            OmniAppMessage::SystemInfo(system_info) => self
                .system_info
                .update(system_info)
                .map(OmniAppMessage::SystemInfo),
            #[cfg(feature = "instax_framer")]
            OmniAppMessage::InstaxFramer(message) => self
                .instax_framer
                .update(message)
                .map(OmniAppMessage::InstaxFramer),
            #[cfg(feature = "ddp")]
            OmniAppMessage::Ddp(message) => self.ddp.update(message).map(OmniAppMessage::Ddp),
        }
    }

    pub fn subscription(&self) -> Subscription<OmniAppMessage> {
        Subscription::batch([
            #[cfg(feature = "counter")]
            Counter::create_auto_increment_subscription().map(OmniAppMessage::CounterEvent),
            // Subscription::run(create_theme_mode_stream),
        ])
    }

    pub(crate) fn start_up_tasks(&self) -> Task<OmniAppMessage> {
        #[allow(clippy::vec_init_then_push)]
        #[allow(unused_mut)]
        let mut start_up_tasks = vec![
            #[cfg(feature = "system_info")]
            SystemInfo::start_up_tasks().map(OmniAppMessage::SystemInfo),
            #[cfg(feature = "instax_framer")]
            self.instax_framer
                .start_up_tasks()
                .map(OmniAppMessage::InstaxFramer),
        ];

        Task::batch(start_up_tasks)
    }
}
