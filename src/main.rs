use iced::{
    alignment::Horizontal,
    widget::{button, checkbox, column, container, row, text, vertical_space},
    Alignment, Element, Length, Size,
};

#[derive(Debug)]
struct Counter {
    value: isize,
    allow_negative: bool,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            value: Default::default(),
            allow_negative: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
enum CounterMessage {
    Increment,
    Decrement,
    Reset,
    ToggleAllowNegative(bool),
}

impl Counter {
    fn view(&self) -> Element<CounterMessage> {
        container(
            column![
                button(text("+").size(25))
                    .width(35)
                    .on_press(CounterMessage::Increment),
                text(self.value).size(65),
                button(text("-").size(25)).width(35).on_press_maybe(
                    if !self.allow_negative && self.value <= 0 {
                        None
                    } else {
                        Some(CounterMessage::Decrement)
                    }
                ),
                vertical_space().height(10),
                container(
                    row![
                        button("Reset")
                            .style(button::danger)
                            .on_press(CounterMessage::Reset),
                        checkbox("Allow negative", self.allow_negative)
                            .on_toggle(CounterMessage::ToggleAllowNegative)
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center)
                )
                .align_x(Horizontal::Center)
                .width(Length::Fill)
            ]
            .align_items(Alignment::Center)
            .height(Length::Fill),
        )
        .align_x(Horizontal::Center)
        .into()
    }

    fn update(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::Increment => self.value += 1,
            CounterMessage::Decrement => {
                if self.value > 0 || self.allow_negative {
                    self.value -= 1
                } else {
                    self.value = 0
                }
            }
            CounterMessage::Reset => self.value = 0,
            CounterMessage::ToggleAllowNegative(allow_negative) => {
                self.allow_negative = allow_negative
            }
        }
    }
}

fn main() -> iced::Result {
    iced::application("iced_counter by @sneu", Counter::update, Counter::view)
        .window_size(Size {
            width: 240.0,
            height: 240.0,
        })
        .run()
}
