use std::ptr::null_mut;
use winapi::um::winuser::{INPUT_u, INPUT, INPUT_KEYBOARD, KEYBDINPUT, SendInput, VK_RETURN};
use std::{thread, time};

pub struct OutputHandler;

impl OutputHandler {
    pub fn new() -> Self {
        OutputHandler
    }

    pub fn type_text(&self, text: &str) {
        for c in text.chars() {
            self.send_char(c);
            thread::sleep(time::Duration::from_millis(10)); // Add a small delay between keystrokes
        }
        self.send_enter(); // Send an Enter key press at the end
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

    fn send_enter(&self) {
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: unsafe { std::mem::zeroed() },
        };
        
        unsafe {
            *input.u.ki_mut() = KEYBDINPUT {
                wVk: VK_RETURN as u16,
                wScan: 0,
                dwFlags: 0,
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
                wVk: VK_RETURN as u16,
                wScan: 0,
                dwFlags: 0x0002, // KEYEVENTF_KEYUP
                time: 0,
                dwExtraInfo: 0,
            };
        }

        unsafe {
            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
        }
    }
}
