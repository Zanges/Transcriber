use iced::widget::{container, Column};
use iced::{Element, Sandbox, Settings};

struct EmptyWindow;

impl Sandbox for EmptyWindow {
    type Message = ();

    fn new() -> Self {
        EmptyWindow
    }

    fn title(&self) -> String {
        String::from("Empty Window")
    }

    fn update(&mut self, _message: Self::Message) {
        // This app has no interactions
    }

    fn view(&self) -> Element<Self::Message> {
        container(Column::new()).into()
    }
}

fn main() -> iced::Result {
    EmptyWindow::run(Settings::default())
}
