use iced::widget::{container, Text};
use iced::{executor, Application, Command, Element, Theme};
use iced::window;
use iced_native::{command::Action, keyboard, Runtime};
use std::sync::Arc;
use tokio::sync::Mutex;

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
                if let Some(window_id) = self.popup_window {
                    self.popup_window = None;
                    window::close(window_id)
                } else {
                    let window_id = window::Id::unique();
                    self.popup_window = Some(window_id);
                    window::new(window::Settings {
                        id: window_id,
                        size: (300, 100),
                        position: window::Position::Centered,
                        ..Default::default()
                    })
                }
            }
            Message::ClosePopup => {
                if let Some(window_id) = self.popup_window.take() {
                    window::close(window_id)
                } else {
                    Command::none()
                }
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
