use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, checkbox, column, container, row, text, PickList, Toggler},
    Alignment, Element, Length, Size, Task, Theme,
};

mod counter_themes;
use counter_themes::theme_from;

mod system_info;
use system_info::{fetch_information, parse_system_info, SystemInfomation};

#[derive(Debug)]
struct Counter {
    value: isize,
    allow_negative: bool,
    dark_theme: Option<bool>,
    theme_name: String,
    system_info: Option<SystemInfomation>,
}

#[derive(Clone, Debug)]
enum CounterMessage {
    Increment,
    Decrement,
    Reset,
    ToggleAllowNegative(bool),
    ToggleDarkTheme(bool),
    SwitchTheme(String),
    NoOp,
    SystemInfoLoaded(SystemInfomation),
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

    fn update(&mut self, message: CounterMessage) -> Task<CounterMessage> {
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
}

fn main() -> iced::Result {
    // while let Some(mode) = dark_light::subscribe().await.unwrap().next().await {
    //     println!("System theme changed: {:?}", mode);
    // }

    iced::application("iced_counter by @sneu", Counter::update, Counter::view)
        .theme(|state| theme_from(&state.theme_name, &state.dark_theme.unwrap_or_default()))
        .window_size(Size {
            width: 512.0,
            height: 240.0,
        })
        .load(|| {
            fetch_information().map(|system_info| CounterMessage::SystemInfoLoaded(system_info))
        })
        .run_with(|| Counter {
            value: Default::default(),
            allow_negative: true,
            dark_theme: Some(Theme::default() == Theme::Dark),
            theme_name: counter_themes::GRUVBOX.to_owned(),
            system_info: None,
        })
}
