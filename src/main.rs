use iced::{
    widget::{button, column, text, vertical_space},
    Alignment, Element, Length, Padding, Size,
};

#[derive(Debug, Default)]
struct Counter {
    value: isize,
}

#[derive(Clone, Debug)]
enum CounterMessage {
    Increment,
    Decrement,
    Reset,
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
            vertical_space().height(10),
            button("Reset")
                .style(button::danger)
                .on_press(CounterMessage::Reset)
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding::from([10, 0]))
        .into()
    }

    fn update(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::Increment => self.value += 1,
            CounterMessage::Decrement => self.value -= 1,
            CounterMessage::Reset => self.value = 0,
        }
    }
}

fn main() -> iced::Result {
    iced::application("iced_counter by @sneu", Counter::update, Counter::view)
        .window_size(Size {
            width: 150.0,
            height: 240.0,
        })
        .run()
}
