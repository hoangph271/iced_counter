use iced::{
    alignment::{Horizontal, Vertical},
    futures::{stream, StreamExt},
    subscription,
    widget::{button, checkbox, column, container, row, text, PickList, Toggler},
    Alignment, Element, Length, Subscription, Task,
};

use crate::counter_themes;
use crate::system_info::{parse_system_info, SystemInfomation};

#[derive(Debug)]
pub struct CounterApp {
    pub value: isize,
    pub allow_negative: bool,
    pub dark_theme: Option<bool>,
    pub theme_name: String,
    pub system_info: Option<SystemInfomation>,
}

#[derive(Clone, Debug)]
pub enum CounterMessage {
    Increment,
    Decrement,
    Reset,
    ToggleAllowNegative(bool),
    ToggleDarkTheme(bool),
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
                    self.dark_theme.unwrap_or_default(),
                    CounterMessage::ToggleDarkTheme
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
            CounterMessage::ToggleDarkTheme(dark_theme) => self.dark_theme = Some(dark_theme),
            CounterMessage::SwitchTheme(theme_name) => self.theme_name = theme_name,
            CounterMessage::SystemInfoLoaded(system_info) => self.system_info = Some(system_info),
            CounterMessage::NoOp => {}
        };

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<CounterMessage> {
        subscription::run(|| {
            stream::once(dark_light::subscribe()).flat_map(|it| {
                if let Ok(stream) = it {
                    stream
                        .map(|mode| match mode {
                            dark_light::Mode::Dark => CounterMessage::ToggleDarkTheme(true),
                            dark_light::Mode::Light => CounterMessage::ToggleDarkTheme(false),
                            dark_light::Mode::Default => CounterMessage::ToggleDarkTheme(true),
                        })
                        .left_stream()
                } else {
                    stream::once(async { CounterMessage::NoOp }).right_stream()
                }
            })
        })
    }
}
