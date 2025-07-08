use iced::{
    alignment::Horizontal,
    time,
    widget::{button, checkbox, column, container, row, text, Toggler},
    Alignment, Element, Length, Subscription, Task,
};
use std::time::Duration;

use crate::omni_app::CounterMessage;

#[derive(Debug, Clone)]
pub(super) struct Counter {
    value: isize,
    allow_negative: bool,
    auto_increment_enabled: bool,
}
impl Counter {
    pub(super) fn update(&mut self, counter_event: CounterMessage) -> Task<CounterMessage> {
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
        }

        Task::none()
    }

    pub(super) fn view(&self) -> Element<'_, CounterMessage> {
        column![
            Toggler::new(self.auto_increment_enabled,)
                .label("Auto increment".to_owned())
                .on_toggle(CounterMessage::ToggleAutoIncrement)
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
            .align_y(Alignment::Center),
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
                .align_y(Alignment::Center)
            )
            .align_x(Horizontal::Center)
            .width(Length::Fill)
        ]
        .align_x(Horizontal::Center)
        .into()
    }

    pub(super) fn create_time_subscription() -> Subscription<CounterMessage> {
        time::every(Duration::from_secs(1)).map(|_| CounterMessage::AutoIncrement)
    }

    pub(crate) fn init() -> Counter {
        Self {
            value: Default::default(),
            allow_negative: true,
            auto_increment_enabled: false,
        }
    }
}
