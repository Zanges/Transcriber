use iced::widget::{button, column, container, pick_list, row, text};
use iced::{executor, Application, Command, Element, Settings, Theme};

use crate::config_handler::Config;

pub struct TranscriberGui {
    config: Config,
    languages: Vec<String>,
    selected_language: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    LanguageSelected(String),
    OpenOptions,
    Exit,
    Ignore,
}

impl Application for TranscriberGui {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Config;

    fn new(config: Config) -> (Self, Command<Message>) {
        let languages = vec![
            "English".to_string(),
            "Spanish".to_string(),
            "French".to_string(),
            "German".to_string(),
            // Add more languages as needed
        ];

        (
            Self {
                config,
                languages,
                selected_language: "English".to_string(), // Default language
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
            }
            Message::OpenOptions => {
                // TODO: Implement opening options dialog
                println!("Open options clicked");
            }
            Message::Exit => {
                return Command::perform(async {}, |_| std::process::exit(0));
            }
            Message::Ignore => {}
        }
        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::subscription::events().map(|event| match event {
            iced::Event::Window(iced::window::Event::CloseRequested) => Message::Exit,
            _ => Message::Ignore,
        })
    }

    fn view(&self) -> Element<Message> {
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

pub fn run_gui(config: Config) -> iced::Result {
    TranscriberGui::run(Settings::with_flags(config))
}
