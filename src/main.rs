use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, checkbox, column, container, row, text, vertical_space, Toggler},
    Alignment, Element, Length, Size, Theme,
};

#[derive(Debug)]
struct Counter {
    value: isize,
    allow_negative: bool,
    dark_theme: bool,
}

#[derive(Clone, Debug)]
enum CounterMessage {
    Increment,
    Decrement,
    Reset,
    ToggleAllowNegative(bool),
    ToggleDarkTheme(bool),
}

impl Counter {
    fn view(&self) -> Element<CounterMessage> {
        container(
            column![
                Toggler::new(
                    Some("Dark theme".into()),
                    self.dark_theme,
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
                vertical_space().height(12),
                container(
                    row![
                        button("Reset")
                            .style(button::danger)
                            .on_press(CounterMessage::Reset),
                        checkbox("Allow negative", self.allow_negative)
                            .on_toggle(CounterMessage::ToggleAllowNegative)
                    ]
                    .spacing(12)
                    .align_items(Alignment::Center)
                )
                .align_x(Horizontal::Center)
                .width(Length::Fill)
            ]
            .align_items(Alignment::Center)
            .height(Length::Fill),
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }

    fn update(&mut self, message: CounterMessage) {
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
            CounterMessage::ToggleDarkTheme(dark_theme) => self.dark_theme = dark_theme,
        }
    }
}

fn main() -> iced::Result {
    iced::application("iced_counter by @sneu", Counter::update, Counter::view)
        .theme(get_theme_from_state)
        .window_size(Size {
            width: 240.0,
            height: 240.0,
        })
        .run_with(|| Counter {
            value: Default::default(),
            allow_negative: true,
            dark_theme: Theme::default() == Theme::Dark,
        })
}

fn get_theme_from_state(state: &Counter) -> Theme {
    if state.dark_theme {
        Theme::Dark
    } else {
        Theme::Light
    }
}
