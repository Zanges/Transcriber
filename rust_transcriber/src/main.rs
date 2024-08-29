use iced::widget::{container, Text};
use iced::{executor, Application, Command, Element, Settings, Theme};
use iced_native::command::Action;
use iced_winit::runtime::Runtime as _;
use iced_winit::window::Id as WindowId;
use std::sync::Arc;
use tokio::sync::Mutex;

struct MessageApp {
    popup_window: Option<WindowId>,
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
                    Command::single(Action::Window(iced_winit::window::Action::Close(window_id)))
                } else {
                    let window_id = WindowId::unique();
                    self.popup_window = Some(window_id);
                    Command::single(Action::Window(iced_winit::window::Action::New(iced_winit::window::Settings {
                        id: window_id,
                        size: (300, 100),
                        position: iced_winit::window::Position::Centered,
                        ..Default::default()
                    })))
                }
            }
            Message::ClosePopup => {
                if let Some(window_id) = self.popup_window.take() {
                    Command::single(Action::Window(iced_winit::window::Action::Close(window_id)))
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

#[tokio::main]
async fn main() -> iced::Result {
    let state = Arc::new(Mutex::new(MessageApp::new().0));

    let runtime = iced_winit::runtime::Runtime::new(iced_winit::settings::Settings::default());

    let global_hotkey = iced_native::keyboard::Hotkey::new(iced_native::keyboard::ModifiersState::empty(), iced_native::keyboard::KeyCode::M);

    runtime.run(
        move |event, _, control_flow| {
            if let iced_winit::event::Event::GlobalHotkey(hotkey) = event {
                if hotkey == global_hotkey {
                    let mut state = state.lock().unwrap();
                    let message = Message::TogglePopup;
                    let command = state.update(message);
                    runtime.spawn(command);
                }
            }
        },
        |_| {},
        |_| {},
    );

    Ok(())
}
