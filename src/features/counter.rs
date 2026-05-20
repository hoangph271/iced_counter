use iced::{
    alignment::Horizontal,
    time,
    widget::{self, button, checkbox, column, container, row, space, text},
    Alignment, Element, Length, Subscription, Task,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug)]
pub enum CounterMessage {
    AutoIncrement,
    ToggleAutoIncrement(bool),
    Increment,
    Decrement,
    Reset,
    ToggleAllowNegative(bool),
    CriticalStateChanged,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct Counter {
    value: isize,
    allow_negative: bool,
    auto_increment_enabled: bool,
}

impl Counter {
    pub(crate) fn update(&mut self, counter_event: CounterMessage) -> Task<CounterMessage> {
        match counter_event {
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
            CounterMessage::CriticalStateChanged => {}
        }

        let should_save_config = matches!(
            counter_event,
            CounterMessage::ToggleAllowNegative(_)
                | CounterMessage::ToggleAutoIncrement(_)
                | CounterMessage::Reset
        );

        if should_save_config {
            Task::done(CounterMessage::CriticalStateChanged)
        } else {
            Task::none()
        }
    }

    pub(crate) fn view(&self) -> Element<'_, CounterMessage> {
        row![
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
            .align_y(Alignment::Center),
            space().width(6),
            container(
                row![
                    column![
                        checkbox(self.allow_negative)
                            .label("Allow negative")
                            .on_toggle(CounterMessage::ToggleAllowNegative),
                        checkbox(self.auto_increment_enabled)
                            .label("Auto increment")
                            .on_toggle(CounterMessage::ToggleAutoIncrement)
                    ],
                    widget::Space::new().width(12),
                    button("Reset")
                        .style(button::danger)
                        .on_press_maybe(if self.value != 0 {
                            Some(CounterMessage::Reset)
                        } else {
                            None
                        }),
                ]
                .spacing(12)
                .align_y(Alignment::Center)
            )
            .align_x(Horizontal::Center)
            .width(Length::Shrink)
        ]
        .align_y(Alignment::Center)
        .into()
    }

    pub(crate) fn subscription(&self) -> Subscription<CounterMessage> {
        if self.auto_increment_enabled {
            time::every(Duration::from_secs(1)).map(|_| CounterMessage::AutoIncrement)
        } else {
            Subscription::none()
        }
    }

    pub(crate) fn init() -> Counter {
        Self {
            value: Default::default(),
            allow_negative: true,
            auto_increment_enabled: false,
        }
    }
}
