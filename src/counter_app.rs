use iced::{
    alignment::{Horizontal, Vertical},
    futures::{stream, Stream, StreamExt},
    subscription,
    time::{self, Duration},
    widget::{button, checkbox, column, container, row, text, PickList, Toggler},
    Alignment, Element, Length, Subscription, Task,
};

use crate::counter_themes;
use crate::system_info::{parse_system_info, SystemInfomation};

#[derive(Debug)]
pub struct CounterApp {
    pub value: isize,
    pub allow_negative: bool,
    pub dark_mode: Option<bool>,
    pub theme_name: String,
    pub system_info: Option<SystemInfomation>,
}

#[derive(Clone, Debug)]
pub enum CounterMessage {
    AutoIncrement,
    Increment,
    Decrement,
    Reset,
    ToggleAllowNegative(bool),
    ToggleDarkMode(bool),
    SwitchTheme(String),
    NoOp,
    SystemInfoLoaded(SystemInfomation),
}

impl CounterApp {
    pub fn view(&self) -> Element<CounterMessage> {
        container(
            column![
                Toggler::new(
                    Some("Dark theme".into()),
                    self.dark_mode.unwrap_or_default(),
                    CounterMessage::ToggleDarkMode
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
                if let Some(system_info) = &self.system_info {
                    text(parse_system_info(system_info))
                } else {
                    text("...")
                }
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
            CounterMessage::AutoIncrement => self.value += 1,
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
            CounterMessage::ToggleDarkMode(dark_mode) => self.dark_mode = Some(dark_mode),
            CounterMessage::SwitchTheme(theme_name) => self.theme_name = theme_name,
            CounterMessage::SystemInfoLoaded(system_info) => self.system_info = Some(system_info),
            CounterMessage::NoOp => {}
        };

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<CounterMessage> {
        Subscription::batch([
            create_time_subscription(),
            subscription::run(create_theme_mode_stream),
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
                    dark_light::Mode::Dark => CounterMessage::ToggleDarkMode(true),
                    dark_light::Mode::Light => CounterMessage::ToggleDarkMode(false),
                    dark_light::Mode::Default => CounterMessage::ToggleDarkMode(true),
                })
                .left_stream()
        } else {
            stream::once(async { CounterMessage::NoOp }).right_stream()
        }
    })
}
