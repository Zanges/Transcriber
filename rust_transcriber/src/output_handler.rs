use std::ptr::null_mut;
use winapi::um::winuser::{INPUT_u, INPUT, INPUT_KEYBOARD, KEYBDINPUT, SendInput, VK_RETURN};
use std::{thread, time};
use crate::config_handler::Config;

pub struct OutputHandler {
    config: Config,
}

impl OutputHandler {
    pub fn new(config: Config) -> Self {
        OutputHandler { config }
    }

    pub fn type_text(&self, text: &str) {
        for c in text.chars() {
            self.send_char(c);
            thread::sleep(time::Duration::from_millis(self.config.keypress_delay));
        }
    }

    fn send_char(&self, c: char) {
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: unsafe { std::mem::zeroed() },
        };
        
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
            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
        }

        // Key up event
        unsafe {
            *input.u.ki_mut() = KEYBDINPUT {
                wVk: 0,
                wScan: c as u16,
                dwFlags: 0x0004 | 0x0002, // KEYEVENTF_UNICODE | KEYEVENTF_KEYUP
                time: 0,
                dwExtraInfo: 0,
            };
        }

        unsafe {
            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
        }
    }
}
