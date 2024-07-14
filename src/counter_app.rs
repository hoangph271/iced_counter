use iced::{
    alignment::{Horizontal, Vertical},
    futures::{stream, Stream, StreamExt},
    time::{self, Duration},
    widget::{button, checkbox, column, container, row, text, vertical_space, PickList, Toggler},
    Alignment, Element, Length, Subscription, Task,
};

use crate::{
    counter_themes::{self, ThemeMode, ALL_THEME_MODES},
    system_info::{system_info_view, SystemInfomation},
};

#[derive(Debug)]
pub struct CounterApp {
    pub value: isize,
    pub allow_negative: bool,
    pub auto_increment_enabled: bool,
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
    ChangeThemeMode(ThemeMode),
    SwitchTheme(String),
    NoOp,
    SystemInfoLoaded(SystemInfomation),
    ChangeSystemThemeMode(ThemeMode),
}

impl CounterApp {
    pub fn view(&self) -> Element<CounterMessage> {
        container(
            column![
                vertical_space().height(4),
                row![
                    PickList::new(
                        ALL_THEME_MODES,
                        Some(&self.application_theme_mode),
                        CounterMessage::ChangeThemeMode
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
                                    CounterMessage::SwitchTheme(theme_name.to_string())
                                }
                                _ => CounterMessage::NoOp,
                            }
                        }
                    ),
                ]
                .spacing(16),
                Toggler::new(
                    "Auto increment".to_owned(),
                    self.auto_increment_enabled,
                    CounterMessage::ToggleAutoIncrement
                )
                .width(Length::Shrink),
                row![
                    button(text("-").size(25)).width(35).on_press_maybe(
                        if !self.allow_negative && self.value <= 0 {
                            None
                        } else {
                            Some(CounterMessage::Decrement)
                        }
                    ),
                    text(self.value).size(65),
                    button(text("+").size(25))
                        .width(35)
                        .on_press(CounterMessage::Increment),
                ]
                .spacing(12)
                .align_items(Alignment::Center),
                container(
                    row![
                        button("Reset")
                            .style(button::danger)
                            .on_press_maybe(if self.value != 0 {
                                Some(CounterMessage::Reset)
                            } else {
                                None
                            }),
                        checkbox("Allow negative", self.allow_negative)
                            .on_toggle(CounterMessage::ToggleAllowNegative)
                    ]
                    .spacing(12)
                    .align_items(Alignment::Center)
                )
                .align_x(Horizontal::Center)
                .width(Length::Fill),
                if let Some(system_info) = &self.system_info {
                    system_info_view(system_info)
                } else {
                    text("...").into()
                },
            ]
            .align_items(Alignment::Center)
            .spacing(12)
            .height(Length::Fill),
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }

    pub fn update(&mut self, message: CounterMessage) -> Task<CounterMessage> {
        match message {
            CounterMessage::ToggleAutoIncrement(enabled) => self.auto_increment_enabled = enabled,
            CounterMessage::AutoIncrement => {
                if self.auto_increment_enabled {
                    self.value += 1
                }
            }
            CounterMessage::Increment => self.value += 1,
            CounterMessage::Decrement => {
                if self.value > 0 || self.allow_negative {
                    self.value -= 1
                }
            }
            CounterMessage::Reset => self.value = 0,
            CounterMessage::ToggleAllowNegative(allow_negative) => {
                self.allow_negative = allow_negative
            }
            CounterMessage::ChangeThemeMode(theme_mode) => self.application_theme_mode = theme_mode,
            CounterMessage::ChangeSystemThemeMode(theme_mode) => {
                self.system_theme_mode = theme_mode;
            }
            CounterMessage::SwitchTheme(theme_name) => self.theme_name = theme_name,
            CounterMessage::SystemInfoLoaded(system_info) => self.system_info = Some(system_info),
            CounterMessage::NoOp => {}
        };

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<CounterMessage> {
        Subscription::batch([
            create_time_subscription(),
            Subscription::run(create_theme_mode_stream),
        ])
    }
}

fn create_time_subscription() -> Subscription<CounterMessage> {
    time::every(Duration::from_secs(1)).map(|_| CounterMessage::AutoIncrement)
}

fn create_theme_mode_stream() -> impl Stream<Item = CounterMessage> {
    stream::once(dark_light::subscribe()).flat_map(|it| {
        if let Ok(stream) = it {
            stream
                .map(|theme_mode| match theme_mode {
                    dark_light::Mode::Dark => {
                        CounterMessage::ChangeSystemThemeMode(ThemeMode::Dark)
                    }
                    dark_light::Mode::Light => {
                        CounterMessage::ChangeSystemThemeMode(ThemeMode::Light)
                    }
                    dark_light::Mode::Default => {
                        CounterMessage::ChangeSystemThemeMode(ThemeMode::SystemDefault)
                    }
                })
                .left_stream()
        } else {
            stream::once(async { CounterMessage::NoOp }).right_stream()
        }
    })
}
