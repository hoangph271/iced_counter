use iced::{
    alignment::{Horizontal, Vertical},
    widget::{column, container, row, vertical_space, PickList},
    Alignment, Element, Length, Subscription, Task,
};

use crate::counter_themes::{self, ThemeMode, ALL_THEME_MODES};

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
    #[cfg(feature = "counter")]
    counter: Counter,
    pub application_theme_mode: ThemeMode,
    pub system_theme_mode: ThemeMode,
    pub theme_name: String,
    #[cfg(feature = "system_info")]
    pub system_info: SystemInfo,
    #[cfg(feature = "instax_framer")]
    pub instax_framer: InstaxFramer,
    #[cfg(feature = "ddp")]
    pub ddp: Ddp,
}

#[derive(Clone, Debug)]
pub enum OmniAppMessage {
    ChangeThemeMode(ThemeMode),
    SwitchTheme(String),
    NoOp,
    #[cfg(feature = "counter")]
    CounterEvent(CounterMessage),
    #[cfg(feature = "system_info")]
    SystemInfo(SystemInfoMessage),
    #[cfg(feature = "instax_framer")]
    InstaxFramer(InstaxFramerMessage),
    #[cfg(feature = "ddp")]
    Ddp(DdpMessage),
}

impl OmniApp {
    pub fn init() -> Self {
        Self {
            #[cfg(feature = "counter")]
            counter: Counter::init(),
            application_theme_mode: ThemeMode::SystemDefault,
            system_theme_mode: counter_themes::get_system_theme_mode(),
            theme_name: counter_themes::GRUVBOX.to_owned(),
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
            column![
                vertical_space().height(4),
                row![
                    PickList::new(
                        ALL_THEME_MODES,
                        Some(&self.application_theme_mode),
                        OmniAppMessage::ChangeThemeMode
                    )
                    .width(Length::Shrink),
                    PickList::new(
                        counter_themes::ALL_THEMES,
                        Some(self.theme_name.as_str()),
                        |theme_name| {
                            match theme_name {
                                counter_themes::DEFAULT
                                | counter_themes::GRUVBOX
                                | counter_themes::SOLARIZED => {
                                    OmniAppMessage::SwitchTheme(theme_name.to_string())
                                }
                                _ => OmniAppMessage::NoOp,
                            }
                        }
                    ),
                ]
                .spacing(16)
                .align_y(Alignment::Center),
            ]
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
            OmniAppMessage::CounterEvent(counter_event) => {
                return self
                    .counter
                    .update(counter_event)
                    .map(OmniAppMessage::CounterEvent)
            }
            OmniAppMessage::ChangeThemeMode(theme_mode) => self.application_theme_mode = theme_mode,
            OmniAppMessage::SwitchTheme(theme_name) => self.theme_name = theme_name,
            #[cfg(feature = "system_info")]
            OmniAppMessage::SystemInfo(system_info) => {
                return self
                    .system_info
                    .update(system_info)
                    .map(OmniAppMessage::SystemInfo)
            }
            #[cfg(feature = "instax_framer")]
            OmniAppMessage::InstaxFramer(message) => {
                return self
                    .instax_framer
                    .update(message)
                    .map(OmniAppMessage::InstaxFramer);
            }
            #[cfg(feature = "ddp")]
            OmniAppMessage::Ddp(message) => {
                return self.ddp.update(message).map(OmniAppMessage::Ddp);
            }
            OmniAppMessage::NoOp => {}
        };

        Task::none()
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
