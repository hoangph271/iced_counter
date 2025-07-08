use iced::{
    alignment::{Horizontal, Vertical},
    widget::{column, container, row, text, vertical_space, PickList},
    Alignment, Element, Length, Subscription, Task,
};

use crate::{
    counter::Counter,
    counter_themes::{self, ThemeMode, ALL_THEME_MODES},
    system_info::{system_info_view, SystemInfomation},
};

#[derive(Debug)]
pub struct OmniApp {
    counter: Counter,
    pub application_theme_mode: ThemeMode,
    pub system_theme_mode: ThemeMode,
    pub theme_name: String,
    pub system_info: Option<SystemInfomation>,
}

#[derive(Clone, Debug)]
pub enum CounterMessage {
    AutoIncrement,
    ToggleAutoIncrement(bool),
    Increment,
    Decrement,
    Reset,
    ToggleAllowNegative(bool),
}

#[derive(Clone, Debug)]
pub enum OmniAppMessage {
    CounterEvent(CounterMessage),
    ChangeThemeMode(ThemeMode),
    SwitchTheme(String),
    NoOp,
    SystemInfoLoaded(SystemInfomation),
}

impl OmniApp {
    pub fn init() -> Self {
        Self {
            counter: Counter::init(),
            application_theme_mode: ThemeMode::SystemDefault,
            system_theme_mode: counter_themes::get_system_theme_mode(),
            theme_name: counter_themes::GRUVBOX.to_owned(),
            system_info: None,
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
                .spacing(16),
                self.counter.view().map(OmniAppMessage::CounterEvent),
                if let Some(system_info) = &self.system_info {
                    system_info_view(system_info)
                } else {
                    text("...").into()
                },
            ]
            .align_x(Alignment::Center)
            .spacing(12)
            .height(Length::Fill),
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }

    pub fn update(&mut self, message: OmniAppMessage) -> Task<OmniAppMessage> {
        match message {
            OmniAppMessage::CounterEvent(counter_event) => {
                return self
                    .counter
                    .update(counter_event)
                    .map(OmniAppMessage::CounterEvent)
            }
            OmniAppMessage::ChangeThemeMode(theme_mode) => self.application_theme_mode = theme_mode,
            OmniAppMessage::SwitchTheme(theme_name) => self.theme_name = theme_name,
            OmniAppMessage::SystemInfoLoaded(system_info) => self.system_info = Some(system_info),
            OmniAppMessage::NoOp => {}
        };

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<OmniAppMessage> {
        Subscription::batch([
            Counter::create_time_subscription().map(OmniAppMessage::CounterEvent),
            // Subscription::run(create_theme_mode_stream),
        ])
    }
}

// ? dark_light::subscribe is removed on dark-light@2.0.0, see https://github.com/rust-dark-light/dark-light/pull/60
// fn create_theme_mode_stream() -> impl Stream<Item = CounterMessage> {
//     stream::once(dark_light::subscribe()).flat_map(|it| {
//         if let Ok(stream) = it {
//             stream
//                 .map(|theme_mode| match theme_mode {
//                     dark_light::Mode::Dark => {
//                         CounterMessage::ChangeSystemThemeMode(ThemeMode::Dark)
//                     }
//                     dark_light::Mode::Light => {
//                         CounterMessage::ChangeSystemThemeMode(ThemeMode::Light)
//                     }
//                     dark_light::Mode::Default => {
//                         CounterMessage::ChangeSystemThemeMode(ThemeMode::SystemDefault)
//                     }
//                 })
//                 .left_stream()
//         } else {
//             stream::once(async { CounterMessage::NoOp }).right_stream()
//         }
//     })
// }
