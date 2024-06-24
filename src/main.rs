use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, checkbox, column, container, row, text, PickList, Toggler},
    Alignment, Element, Length, Size, Theme,
};

mod counter_themes {
    pub const DEFAULT: &str = "Default";
    pub const GRUVBOX: &str = "Gruvbox";
    pub const SOLARIZED: &str = "Solarized";
}

#[derive(Debug)]
struct Counter {
    value: isize,
    allow_negative: bool,
    dark_theme: Option<bool>,
    theme_name: Option<String>,
}

#[derive(Clone, Debug)]
enum CounterMessage {
    Increment,
    Decrement,
    Reset,
    ToggleAllowNegative(bool),
    ToggleDarkTheme(bool),
    SwitchTheme(Option<String>),
    NoOp,
}

impl Counter {
    fn view(&self) -> Element<CounterMessage> {
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
                            .on_press(CounterMessage::Reset),
                        checkbox("Allow negative", self.allow_negative)
                            .on_toggle(CounterMessage::ToggleAllowNegative)
                    ]
                    .spacing(12)
                    .align_items(Alignment::Center)
                )
                .align_x(Horizontal::Center)
                .width(Length::Fill),
                PickList::new(
                    vec![
                        counter_themes::DEFAULT,
                        counter_themes::GRUVBOX,
                        counter_themes::SOLARIZED
                    ],
                    self.theme_name.as_deref(),
                    |theme_name| {
                        match theme_name {
                            counter_themes::GRUVBOX | counter_themes::SOLARIZED => {
                                CounterMessage::SwitchTheme(Some(theme_name.to_string()))
                            }
                            counter_themes::DEFAULT => CounterMessage::SwitchTheme(None),
                            _ => CounterMessage::NoOp,
                        }
                    }
                ),
            ]
            .align_items(Alignment::Center)
            .spacing(12)
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
            CounterMessage::ToggleDarkTheme(dark_theme) => self.dark_theme = Some(dark_theme),
            CounterMessage::SwitchTheme(theme_name) => self.theme_name = theme_name,
            CounterMessage::NoOp => {}
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
            dark_theme: Some(Theme::default() == Theme::Dark),
            theme_name: Some(counter_themes::DEFAULT.to_owned()),
        })
}

fn get_theme_from_state(state: &Counter) -> Theme {
    let dark_theme = state.dark_theme.is_some_and(|it| it);

    match (state.theme_name.as_deref(), dark_theme) {
        (Some(counter_themes::GRUVBOX), true) => Theme::GruvboxDark,
        (Some(counter_themes::GRUVBOX), false) => Theme::GruvboxLight,
        (Some(counter_themes::SOLARIZED), true) => Theme::SolarizedDark,
        (Some(counter_themes::SOLARIZED), false) => Theme::SolarizedLight,
        (_, true) => Theme::Dark,
        (_, false) => Theme::Light,
    }
}
