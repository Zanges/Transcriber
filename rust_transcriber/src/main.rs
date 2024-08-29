use iced::keyboard::{Event, KeyCode};
use iced::widget::{container, Text};
use iced::{executor, Application, Command, Element, Event as IcedEvent, Settings, Subscription, Theme};

struct MessageApp {
    show_message: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleMessage,
    EventOccurred(IcedEvent),
}

impl Application for MessageApp {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            MessageApp {
                show_message: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Message Box App")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleMessage => {
                self.show_message = !self.show_message;
                Command::none()
            }
            Message::EventOccurred(event) => {
                if let IcedEvent::Keyboard(Event::KeyPressed { key_code, .. }) = event {
                    if key_code == KeyCode::M {
                        return self.update(Message::ToggleMessage);
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content: Element<_> = if self.show_message {
            Text::new("Hello! This is a message box.").into()
        } else {
            Text::new("Press 'M' to show/hide the message.").into()
        };

        container(content).center_x().center_y().into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::subscription::events().map(Message::EventOccurred)
    }
}

fn main() -> iced::Result {
    MessageApp::run(Settings::default())
}
