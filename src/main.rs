use iced::{
    widget::{button, column, text},
    Alignment, Element, Size,
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

impl Counter {
    fn view(&self) -> Element<CounterMessage> {
        column![
            button(text("+").size(25))
                .width(35)
                .on_press(CounterMessage::Increment),
            text(self.value).size(65),
            button(text("-").size(25))
                .width(35)
                .on_press(CounterMessage::Decrement),
        ]
        .align_items(Alignment::Center)
        .width(150)
        .into()
    }

    fn update(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::Increment => self.value += 1,
            CounterMessage::Decrement => self.value -= 1,
        }
    }
}

fn main() -> iced::Result {
    iced::application("iced_counter by @sneu", Counter::update, Counter::view)
        .window_size(Size {
            width: 150.0,
            height: 200.0,
        })
        .run()
}
