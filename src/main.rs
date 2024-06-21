use iced::{
    widget::{button, column, text},
    Element, Sandbox, Settings, Size,
};

#[derive(Debug, Default)]
struct Counter {
    value: isize,
}

#[derive(Clone, Debug)]
enum CounterMessage {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = CounterMessage;

    fn view(&self) -> Element<CounterMessage> {
        column![
            button("+").on_press(CounterMessage::Increment),
            text(self.value).size(65),
            button("-").on_press(CounterMessage::Decrement)
        ]
        .into()
    }

    fn update(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::Increment => self.value += 1,
            CounterMessage::Decrement => self.value -= 1,
        }
    }

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "iced_counter by @sneu".into()
    }
}

fn main() -> iced::Result {
    let mut settings = Settings::default();

    settings.window.size = Size {
        width: 150.0,
        height: 150.0,
    };

    let settings = settings;

    Counter::run(settings)
}
