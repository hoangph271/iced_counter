use iced::{
    Alignment, Element, Length, Subscription, Task, Theme,
    alignment::{Horizontal, Vertical},
    widget::{self, column, container},
    window,
};
use rfd::{MessageDialog, MessageLevel};
use serde::{Deserialize, Serialize};

use crate::constants::APP_NAME;
#[cfg(feature = "omni_themes")]
use crate::features::omni_themes::{OmniThemes, OmniThemesMessage};
#[cfg(feature = "system_info")]
use crate::features::system_info::{SystemInfo, SystemInfoMessage};

#[cfg(feature = "counter")]
use crate::features::counter::{Counter, CounterMessage};

#[cfg(feature = "instax_framer")]
use crate::features::instax_framer::{InstaxFramer, InstaxFramerMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct OmniAppConfig {
    #[cfg(feature = "counter")]
    counter: Counter,
    #[cfg(feature = "omni_themes")]
    omni_themes: OmniThemes,
    #[cfg(feature = "instax_framer")]
    instax_framer: InstaxFramer,
}

impl PartialEq for OmniAppConfig {
    fn eq(&self, other: &Self) -> bool {
        if self.counter != other.counter {
            return false;
        }

        if self.omni_themes.dark_theme != other.omni_themes.dark_theme
            && self.omni_themes.light_theme != other.omni_themes.light_theme
            && self.omni_themes.application_theme_mode != other.omni_themes.application_theme_mode
        {
            return false;
        }

        if self.instax_framer.selected_file != other.instax_framer.selected_file {
            return false;
        }

        true
    }
}

impl Default for OmniAppConfig {
    fn default() -> Self {
        Self {
            #[cfg(feature = "counter")]
            counter: Counter::init(),
            #[cfg(feature = "omni_themes")]
            omni_themes: OmniThemes::init(),
            #[cfg(feature = "instax_framer")]
            instax_framer: InstaxFramer::init(),
        }
    }
}

#[derive(Debug)]
pub(super) struct OmniApp {
    // TODO: Maybe use a hash instead of storing the config directly
    last_saved_config: Option<OmniAppConfig>,
    #[cfg(feature = "omni_themes")]
    pub omni_themes: OmniThemes,
    #[cfg(feature = "counter")]
    counter: Counter,
    #[cfg(feature = "system_info")]
    pub system_info: SystemInfo,
    #[cfg(feature = "instax_framer")]
    pub instax_framer: InstaxFramer,
}

#[derive(Clone, Debug)]
pub enum OmniAppMessage {
    NoOp,
    ConfigLoaded(OmniAppConfig),
    SavingConfigRequested,
    SavingConfigFailed(String),
    CloseRequested,
    #[cfg(feature = "counter")]
    CounterEvent(CounterMessage),
    #[cfg(feature = "system_info")]
    SystemInfo(SystemInfoMessage),
    #[cfg(feature = "instax_framer")]
    InstaxFramer(InstaxFramerMessage),
    #[cfg(feature = "omni_themes")]
    OmniThemes(OmniThemesMessage),
}

impl OmniApp {
    pub fn init() -> Self {
        Self {
            last_saved_config: None,
            #[cfg(feature = "omni_themes")]
            omni_themes: OmniThemes::init(),
            #[cfg(feature = "counter")]
            counter: Counter::init(),
            #[cfg(feature = "system_info")]
            system_info: SystemInfo::init(),
            #[cfg(feature = "instax_framer")]
            instax_framer: InstaxFramer::init(),
        }
    }

    pub fn view(&'_ self) -> Element<'_, OmniAppMessage> {
        container(
            column![widget::Space::new().height(4)]
                .push(
                    #[cfg(feature = "omni_themes")]
                    Some(self.omni_themes.view().map(OmniAppMessage::OmniThemes)),
                    #[cfg(not(feature = "omni_themes"))]
                    None::<Element<'_, OmniAppMessage>>,
                )
                .push(
                    #[cfg(feature = "counter")]
                    Some(self.counter.view().map(OmniAppMessage::CounterEvent)),
                    #[cfg(not(feature = "counter"))]
                    None::<Element<'_, OmniAppMessage>>,
                )
                .push(
                    #[cfg(feature = "system_info")]
                    Some(self.system_info.view().map(OmniAppMessage::SystemInfo)),
                    #[cfg(not(feature = "system_info"))]
                    None::<Element<'_, OmniAppMessage>>,
                )
                .push(
                    #[cfg(feature = "instax_framer")]
                    Some(self.instax_framer.view().map(OmniAppMessage::InstaxFramer)),
                    #[cfg(not(feature = "instax_framer"))]
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

    fn load_config(&mut self) -> Task<OmniAppMessage> {
        Task::perform(
            async move { confy::load(APP_NAME, None).unwrap_or_default() },
            OmniAppMessage::ConfigLoaded,
        )
    }

    fn save_config(&mut self) -> Task<OmniAppMessage> {
        let app_config = OmniAppConfig {
            #[cfg(feature = "counter")]
            counter: self.counter.clone(),
            #[cfg(feature = "omni_themes")]
            omni_themes: self.omni_themes.clone(),
            #[cfg(feature = "instax_framer")]
            instax_framer: self.instax_framer.clone(),
        };

        if let Some(last_saved_config) = &self.last_saved_config
            && last_saved_config == &app_config
        {
            return Task::done(OmniAppMessage::NoOp);
        }

        Task::perform(
            async move { confy::store(APP_NAME, None, app_config) },
            |result| match result {
                Ok(()) => {
                    println!(
                        "Config saved {:?}",
                        confy::get_configuration_file_path(APP_NAME, None)
                    );

                    OmniAppMessage::NoOp
                }
                Err(e) => OmniAppMessage::SavingConfigFailed(e.to_string()),
            },
        )
    }

    pub fn update(&mut self, message: OmniAppMessage) -> Task<OmniAppMessage> {
        match message {
            OmniAppMessage::NoOp => Task::done(OmniAppMessage::NoOp),
            // Config operations
            OmniAppMessage::CloseRequested => Task::done(OmniAppMessage::SavingConfigRequested)
                .chain(window::latest().and_then(window::close::<OmniAppMessage>)),
            OmniAppMessage::ConfigLoaded(app_config) => {
                self.last_saved_config = Some(app_config.clone());

                self.counter = app_config.counter;
                self.omni_themes = app_config.omni_themes;
                self.instax_framer = app_config.instax_framer;

                let mut tasks: Vec<Task<OmniAppMessage>> = vec![];

                if let Some(path) = self.instax_framer.selected_file.clone() {
                    tasks.push(
                        self.instax_framer
                            .update(InstaxFramerMessage::ImagePicked(path))
                            .map(OmniAppMessage::InstaxFramer),
                    );
                }

                Task::batch(tasks)
            }
            OmniAppMessage::SavingConfigFailed(message) => {
                let dialog = MessageDialog::new()
                    .set_title("Failed to save config")
                    .set_description(message)
                    .set_level(MessageLevel::Error);

                let _ = dialog.show();

                Task::done(OmniAppMessage::NoOp)
            }
            OmniAppMessage::SavingConfigRequested => self.save_config(),
            // Feature-specific message handlers
            #[cfg(feature = "counter")]
            OmniAppMessage::CounterEvent(counter_event) => {
                let save_task = if let CounterMessage::CriticalStateChanged = counter_event {
                    Task::done(OmniAppMessage::SavingConfigRequested)
                } else {
                    Task::none()
                };

                let task = self
                    .counter
                    .update(counter_event)
                    .map(OmniAppMessage::CounterEvent);

                task.chain(save_task)
            }
            #[cfg(feature = "omni_themes")]
            OmniAppMessage::OmniThemes(message) => {
                let task = self
                    .omni_themes
                    .update(message)
                    .map(OmniAppMessage::OmniThemes);

                task.chain(Task::done(OmniAppMessage::SavingConfigRequested))
            }
            #[cfg(feature = "system_info")]
            OmniAppMessage::SystemInfo(system_info) => self
                .system_info
                .update(system_info)
                .map(OmniAppMessage::SystemInfo),
            #[cfg(feature = "instax_framer")]
            OmniAppMessage::InstaxFramer(message) => {
                let should_save_config = matches!(message, InstaxFramerMessage::ImagePicked(_));

                let task = self
                    .instax_framer
                    .update(message)
                    .map(OmniAppMessage::InstaxFramer);

                let save_task = if should_save_config {
                    Task::done(OmniAppMessage::SavingConfigRequested)
                } else {
                    Task::none()
                };

                task.chain(save_task)
            }
        }
    }

    pub fn subscription(&self) -> Subscription<OmniAppMessage> {
        Subscription::batch([
            iced::event::listen().filter_map(|event| match event {
                iced::Event::Window(iced::window::Event::CloseRequested) => {
                    Some(OmniAppMessage::CloseRequested)
                }
                _ => None,
            }),
            #[cfg(feature = "counter")]
            self.counter
                .subscription()
                .map(OmniAppMessage::CounterEvent),
            #[cfg(feature = "omni_themes")]
            self.omni_themes
                .subscription()
                .map(OmniAppMessage::OmniThemes),
        ])
    }

    pub(crate) fn start_up_tasks(&mut self) -> Task<OmniAppMessage> {
        #[allow(clippy::vec_init_then_push)]
        #[allow(unused_mut)]
        let mut start_up_tasks = vec![
            #[cfg(feature = "system_info")]
            SystemInfo::start_up_tasks().map(OmniAppMessage::SystemInfo),
            #[cfg(feature = "instax_framer")]
            self.instax_framer
                .start_up_tasks()
                .map(OmniAppMessage::InstaxFramer),
            #[cfg(feature = "omni_themes")]
            self.omni_themes
                .start_up_tasks()
                .map(OmniAppMessage::OmniThemes),
            self.load_config(),
        ];

        Task::batch(start_up_tasks)
    }

    pub fn theme(&self) -> Theme {
        #[cfg(feature = "omni_themes")]
        {
            self.omni_themes.theme()
        }
        #[cfg(not(feature = "omni_themes"))]
        {
            Theme::GruvboxLight
        }
    }
}
