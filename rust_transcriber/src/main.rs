use iced::widget::{container, Text};
use iced::{executor, Application, Command, Element, Theme};
use iced::window;

struct MessageApp {
    popup_window: Option<window::Id>,
}

#[derive(Debug, Clone)]
enum Message {
    TogglePopup,
    ClosePopup,
}

impl Application for MessageApp {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            MessageApp {
                popup_window: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Message Box App")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TogglePopup => {
                if self.popup_window.is_some() {
                    self.popup_window = None;
                    window::close()
                } else {
                    let window_id = window::Id::unique();
                    self.popup_window = Some(window_id);
                    window::create(window::Settings {
                        size: (300, 100),
                        position: window::Position::Centered,
                        ..Default::default()
                    })
                }
            }
            Message::ClosePopup => {
                self.popup_window = None;
                window::close()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = Text::new("Press 'M' to show/hide the popup window.");
        container(content).center_x().center_y().into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::subscription::events().map(|event| {
            if let iced::Event::Window(iced::window::Event::CloseRequested) = event {
                Message::ClosePopup
            } else {
                Message::TogglePopup
            }
        })
    }
}

fn main() -> iced::Result {
    MessageApp::run(iced::Settings::default())
}
