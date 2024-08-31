
use winapi::um::winuser::{INPUT, INPUT_KEYBOARD, KEYBDINPUT, SendInput};
use std::{thread, time};
pub struct OutputHandler {
    word_delay: u64,
    key_event_delay: u64,
}

impl OutputHandler {
    pub fn new(word_delay: u64, key_event_delay: u64) -> Self {
        OutputHandler { word_delay, key_event_delay }
    }

    fn send_char(&self, c: char) {
        let mut input: INPUT = unsafe { std::mem::zeroed() };

        input.type_ = INPUT_KEYBOARD;
        unsafe {
            *input.u.ki_mut() = KEYBDINPUT {
                wVk: 0,
                wScan: c as u16,
                dwFlags: 0x0004, // KEYEVENTF_UNICODE
                time: 0,
                dwExtraInfo: 0,
            };
        }

        unsafe {
            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32)
        };

        // Add a configurable delay after sending the input
        thread::sleep(time::Duration::from_millis(self.key_event_delay));
    }

    pub fn type_text(&self, text: &str) { //TODO: sometimes the text is not typed correctly, need to fix
        println!("Starting to type text ({} characters)", text.len());
        let words: Vec<&str> = text.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            // Add a configurable delay before each word
            thread::sleep(time::Duration::from_millis(self.word_delay));
            
            for c in word.chars() {
                self.send_char(c);
            }
            if i < words.len() - 1 {
                self.send_char(' ');
            }
        }
        println!("Finished typing text");
    }
}
