
use winapi::um::winuser::{INPUT, INPUT_KEYBOARD, KEYBDINPUT, SendInput};
use std::{thread, time};
pub struct OutputHandler {
    keypress_delay: u64,
}

impl OutputHandler {
    pub fn new(keypress_delay: u64) -> Self {
        OutputHandler { keypress_delay }
    }

    pub fn type_text(&self, text: &str) {
        println!("Starting to type text with {} characters", text.len());
        for (i, c) in text.chars().enumerate() {
            println!("Typing character {} of {}: '{}'", i + 1, text.len(), c);
            self.send_char(c);
            thread::sleep(time::Duration::from_millis(self.keypress_delay));
        }
        println!("Finished typing text");
    }

    fn send_char(&self, c: char) {
        println!("Sending character: '{}'", c);
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

        let result = unsafe {
            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32)
        };
        println!("SendInput result: {}", result);

        // Add a small delay after sending the input
        thread::sleep(time::Duration::from_millis(10));

        println!("Character '{}' should have been typed", c);
    }

    pub fn type_text(&self, text: &str) {
        println!("Starting to type text with {} characters", text.len());
        let words: Vec<&str> = text.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            if let Some(first_char) = word.chars().next() {
                println!("Typing first character of word {} of {}: '{}'", i + 1, words.len(), first_char);
                self.send_char(first_char);
            }
            for c in word.chars().skip(1) {
                self.send_char(c);
            }
            if i < words.len() - 1 {
                self.send_char(' ');
            }
            thread::sleep(time::Duration::from_millis(self.keypress_delay));
        }
        println!("Finished typing text");
    }
}
