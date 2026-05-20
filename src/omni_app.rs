#[cfg(feature = "config")]
use crate::constants::APP_NAME;
use iced::{
    Alignment, Element, Length, Subscription, Task, Theme,
    alignment::{Horizontal, Vertical},
    widget::{self, column, container},
    window,
};
#[cfg(feature = "config")]
use rfd::{MessageDialog, MessageLevel};

#[cfg(feature = "omni_themes")]
use crate::features::omni_themes::{OmniThemes, OmniThemesMessage};
#[cfg(feature = "system_info")]
use crate::features::system_info::{SystemInfo, SystemInfoMessage};

#[cfg(feature = "counter")]
use crate::features::counter::{Counter, CounterMessage};

#[cfg(feature = "instax_framer")]
use crate::features::instax_framer::{InstaxFramer, InstaxFramerMessage};

#[cfg(feature = "config")]
use crate::features::config::OmniAppConfig;

#[derive(Debug)]
pub(super) struct OmniApp {
    // TODO: Maybe use a hash instead of storing the config directly
    #[cfg(feature = "config")]
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

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum OmniAppMessage {
    NoOp,
    #[cfg(feature = "config")]
    ConfigLoaded(OmniAppConfig),
    #[cfg(feature = "config")]
    SavingConfigRequested,
    #[cfg(feature = "config")]
    SavingConfigFailed(String),
    #[cfg(feature = "config")]
    LoadingConfigFailed(String),
    CloseRequested,
    #[cfg(feature = "counter")]
    CounterEvent(CounterMessage),
    #[cfg(feature = "system_info")]
    SystemInfo(SystemInfoMessage),
    #[cfg(feature = "instax_framer")]
    InstaxFramer(InstaxFramerMessage),
    #[cfg(feature = "omni_themes")]
    OmniThemes(OmniThemesMessage),
    TerminateImmediately,
}

impl OmniApp {
    pub fn init() -> Self {
        Self {
            #[cfg(feature = "config")]
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

    #[cfg(feature = "config")]
    fn load_config(&mut self) -> Task<OmniAppMessage> {
        Task::perform(
            async move { confy::load(APP_NAME, None) },
            |result| match result {
                Ok(config) => OmniAppMessage::ConfigLoaded(config),
                Err(e) => OmniAppMessage::LoadingConfigFailed(e.to_string()),
            },
        )
    }

    #[cfg(feature = "config")]
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
                    // TODO: replace with proper logging once a logger is wired up
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
            OmniAppMessage::NoOp => Task::none(),
            OmniAppMessage::TerminateImmediately => {
                window::latest().and_then(window::close::<OmniAppMessage>)
            }
            // Config operations
            OmniAppMessage::CloseRequested => {
                #[cfg(feature = "config")]
                {
                    self.save_config()
                        .chain(window::latest().and_then(window::close::<OmniAppMessage>))
                }

                #[cfg(not(feature = "config"))]
                {
                    window::latest().and_then(window::close::<OmniAppMessage>)
                }
            }
            #[cfg(feature = "config")]
            OmniAppMessage::ConfigLoaded(app_config) => {
                self.last_saved_config = Some(app_config.clone());

                #[cfg(feature = "counter")]
                {
                    self.counter = app_config.counter;
                }

                #[cfg(feature = "omni_themes")]
                {
                    self.omni_themes = app_config.omni_themes;
                }

                #[cfg(feature = "instax_framer")]
                {
                    self.instax_framer = app_config.instax_framer;
                }

                #[allow(unused_mut)]
                let mut tasks: Vec<Task<OmniAppMessage>> = vec![];

                #[cfg(feature = "instax_framer")]
                if let Some(path) = self.instax_framer.selected_file.clone() {
                    tasks.push(
                        self.instax_framer
                            .update(InstaxFramerMessage::ImagePicked(path))
                            .map(OmniAppMessage::InstaxFramer),
                    );
                }

                Task::batch(tasks)
            }
            #[cfg(feature = "config")]
            OmniAppMessage::SavingConfigFailed(message) => {
                let dialog = MessageDialog::new()
                    .set_title("Failed to save config")
                    .set_description(message)
                    .set_level(MessageLevel::Error);

                let _ = dialog.show();

                Task::done(OmniAppMessage::NoOp)
            }
            #[cfg(feature = "config")]
            OmniAppMessage::LoadingConfigFailed(message) => {
                let dialog = MessageDialog::new()
                    .set_title("Failed to load config")
                    .set_description(format!(
                        "Error: {}\nContinue with default settings?",
                        message
                    ))
                    .set_level(MessageLevel::Error)
                    .set_buttons(rfd::MessageButtons::YesNo);

                match dialog.show() {
                    rfd::MessageDialogResult::Yes => Task::done(OmniAppMessage::NoOp),
                    rfd::MessageDialogResult::No => {
                        Task::done(OmniAppMessage::TerminateImmediately)
                    }
                    _ => unreachable!(),
                }
            }
            #[cfg(feature = "config")]
            OmniAppMessage::SavingConfigRequested => self.save_config(),
            // Feature-specific message handlers
            #[cfg(feature = "counter")]
            OmniAppMessage::CounterEvent(message) => {
                let save_task = if let CounterMessage::CriticalStateChanged = message {
                    Task::done(OmniAppMessage::SavingConfigRequested)
                } else {
                    Task::none()
                };

                let task = self
                    .counter
                    .update(message)
                    .map(OmniAppMessage::CounterEvent);

                task.chain(save_task)
            }
            #[cfg(feature = "omni_themes")]
            OmniAppMessage::OmniThemes(message) => {
                let task = self
                    .omni_themes
                    .update(message.clone())
                    .map(OmniAppMessage::OmniThemes);

                let save_task = if let OmniThemesMessage::CriticalStateChanged = message {
                    Task::done(OmniAppMessage::SavingConfigRequested)
                } else {
                    Task::none()
                };

                task.chain(save_task)
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
        // TODO: load_config runs concurrently with other startup tasks via Task::batch; if those
        // tasks dispatch state mutations, they can interleave with ConfigLoaded overwriting the
        // same fields. Consider making load_config the first sequential step if this causes issues.
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
            #[cfg(feature = "config")]
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
