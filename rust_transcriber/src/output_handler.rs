
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
        for c in text.chars() {
            self.send_char(c);
            thread::sleep(time::Duration::from_millis(self.keypress_delay));
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
