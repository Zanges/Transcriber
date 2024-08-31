use iced::widget::{button, column, container, pick_list, row, text};
use iced::{executor, Application, Command, Element, Settings, Theme};

use crate::config_handler::Config;
use crate::config_gui::{ConfigGui, ConfigMessage};

pub struct TranscriberGui {
    config: Config,
    languages: Vec<String>,
    selected_language: String,
    config_gui: Option<ConfigGui>,
}

#[derive(Debug, Clone)]
pub enum Message {
    LanguageSelected(String),
    OpenOptions,
    CloseOptions,
    ConfigMessage(ConfigMessage),
    Exit,
    EventOccurred(iced::Event),
}

impl Application for TranscriberGui {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Config;

    fn new(config: Config) -> (Self, Command<Message>) {
        let languages = vec![
            "Automatic".to_string(),
            "English".to_string(),
            "Spanish".to_string(),
            "French".to_string(),
            "German".to_string(),
            // Add more languages as needed
        ];

        let selected_language = if languages.contains(&config.language) {
            config.language.clone()
        } else {
            "Automatic".to_string() // Default to Automatic if the config language is not in the list
        };

        (
            Self {
                config,
                languages,
                selected_language,
                config_gui: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rust Transcriber")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LanguageSelected(language) => {
                self.selected_language = language;
                self.config.language = self.selected_language.clone();
                // Save the updated config
                if let Err(e) = self.config.save() {
                    eprintln!("Failed to save config: {}", e);
                }
                Command::none()
            }
            Message::OpenOptions => {
                self.config_gui = Some(ConfigGui::new(self.config.clone()));
                Command::none()
            }
            Message::CloseOptions => {
                self.config_gui = None;
                Command::none()
            }
            Message::ConfigMessage(config_message) => {
                if let Some(config_gui) = &mut self.config_gui {
                    if let Err(e) = config_gui.update(config_message) {
                        eprintln!("Failed to update config: {}", e);
                    }
                    if let ConfigMessage::SaveConfig = config_message {
                        self.config = config_gui.config.clone();
                    }
                }
                Command::none()
            }
            Message::Exit => iced::window::close(),
            Message::EventOccurred(event) => {
                if let iced::Event::Window(iced::window::Event::CloseRequested) = event {
                    iced::window::close()
                } else {
                    Command::none()
                }
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::subscription::events().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Message> {
        if let Some(config_gui) = &self.config_gui {
            let config_view = config_gui.view().map(Message::ConfigMessage);
            let close_button = button("Close Options").on_press(Message::CloseOptions);
            
            column![config_view, close_button].spacing(20).into()
        } else {
            let language_picker = pick_list(
                &self.languages,
                Some(self.selected_language.clone()),
                Message::LanguageSelected,
            )
            .placeholder("Select language");

            let options_button = button("Open Options").on_press(Message::OpenOptions);

            let content = column![
                row![text("Language:").width(100), language_picker].spacing(10),
                options_button,
            ]
            .spacing(20);

            container(content)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .center_x()
                .center_y()
                .into()
        }
    }
}

pub fn run_gui(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    TranscriberGui::run(Settings::with_flags(config))?;
    Ok(())
}
