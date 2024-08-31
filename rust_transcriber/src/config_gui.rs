use iced::widget::{button, column, container, row, text, text_input};
use iced::{Element, Length, Alignment};

use crate::config_handler::Config;

#[derive(Debug, Clone)]
pub enum ConfigMessage {
    HotkeyChanged(String),
    OpenAIApiKeyChanged(String),
    WordDelayChanged(String),
    KeyEventDelayChanged(String),
    SaveConfig,
}

pub struct ConfigGui<'a> {
    config: &'a mut Config,
    hotkey: String,
    openai_api_key: String,
    word_delay: String,
    key_event_delay: String,
}

impl<'a> ConfigGui<'a> {
    pub fn new(config: &'a mut Config) -> Self {
        Self {
            hotkey: config.hotkey.clone(),
            openai_api_key: config.openai_api_key.clone(),
            word_delay: config.word_delay.to_string(),
            key_event_delay: config.key_event_delay.to_string(),
            config,
        }
    }

    pub fn update(&mut self, message: &ConfigMessage) -> Result<(), Box<dyn std::error::Error>> {
        match message {
            ConfigMessage::HotkeyChanged(value) => self.hotkey = value.clone(),
            ConfigMessage::OpenAIApiKeyChanged(value) => self.openai_api_key = value.clone(),
            ConfigMessage::WordDelayChanged(value) => self.word_delay = value.clone(),
            ConfigMessage::KeyEventDelayChanged(value) => self.key_event_delay = value.clone(),
            ConfigMessage::SaveConfig => {
                self.config.hotkey = self.hotkey.clone();
                self.config.openai_api_key = self.openai_api_key.clone();
                self.config.word_delay = self.word_delay.parse().unwrap_or(self.config.word_delay);
                self.config.key_event_delay = self.key_event_delay.parse().unwrap_or(self.config.key_event_delay);
                self.config.save()?;
            }
        }
        Ok(())
    }

    pub fn view(&self) -> Element<ConfigMessage> {
        let content = column![
            row![
                text("Hotkey:").width(Length::Fixed(150.0)),
                text_input("Enter hotkey", &self.hotkey)
                    .on_input(ConfigMessage::HotkeyChanged)
            ].spacing(10).align_items(Alignment::Center),
            row![
                text("OpenAI API Key:").width(Length::Fixed(150.0)),
                text_input("Enter API key", &self.openai_api_key)
                    .on_input(ConfigMessage::OpenAIApiKeyChanged)
            ].spacing(10).align_items(Alignment::Center),
            row![
                text("Word Delay (ms):").width(Length::Fixed(150.0)),
                text_input("Enter word delay", &self.word_delay)
                    .on_input(ConfigMessage::WordDelayChanged)
            ].spacing(10).align_items(Alignment::Center),
            row![
                text("Key Event Delay (ms):").width(Length::Fixed(150.0)),
                text_input("Enter key event delay", &self.key_event_delay)
                    .on_input(ConfigMessage::KeyEventDelayChanged)
            ].spacing(10).align_items(Alignment::Center),
            button("Save Configuration").on_press(ConfigMessage::SaveConfig)
        ]
        .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
