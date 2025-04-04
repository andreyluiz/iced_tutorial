use iced::{
    Font, Length,
    alignment::Horizontal,
    font::Family,
    widget::{Toggler, column, text::Shaping, toggler},
};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    DoNothing,
    Update(bool),
}

#[derive(Default)]
struct MyApp {
    is_checked: bool,
}

impl MyApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::DoNothing => {}
            Message::Update(b) => self.is_checked = b,
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            Toggler::new(self.is_checked)
                .label("Construct from struct")
                .on_toggle(|_| Message::DoNothing),
            toggler(self.is_checked)
                .label("Construct from function")
                .on_toggle(|_| Message::DoNothing),
            toggler(self.is_checked)
                .label("Functional toggler")
                .on_toggle(|b| Message::Update(b)),
            toggler(self.is_checked)
                .label("Shorter parameter")
                .on_toggle(|b| Message::Update(b)),
            toggler(self.is_checked)
                .label("Larger button")
                .size(30)
                .on_toggle(|_| Message::DoNothing),
            toggler(self.is_checked)
                .label("Different font")
                .font(Font {
                    family: Family::Fantasy,
                    ..Font::DEFAULT
                })
                .on_toggle(|_| Message::DoNothing),
            toggler(self.is_checked)
                .label("Larger text")
                .text_size(24)
                .on_toggle(|_| Message::DoNothing),
            toggler(self.is_checked)
                .label("Special character 😊")
                .text_shaping(Shaping::Advanced),
            toggler(self.is_checked)
                .label("Space between button and text")
                .spacing(30),
            toggler(self.is_checked)
                .label("Centered text")
                .width(Length::Fill)
                .text_alignment(Horizontal::Center),
        ]
        .into()
    }
}
