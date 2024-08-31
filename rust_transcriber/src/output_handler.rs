
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
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: unsafe { std::mem::zeroed() },
        };
        
        // Key down event
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
        println!("SendInput (key down) result: {}", result);

        // Add a small delay between key down and key up
        thread::sleep(time::Duration::from_millis(5));

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

        let result = unsafe {
            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32)
        };
        println!("SendInput (key up) result: {}", result);

        // Check if the character was actually typed
        // This is a placeholder - you'll need to implement a way to check the actual output
        println!("Character '{}' should have been typed", c);
    }
}
