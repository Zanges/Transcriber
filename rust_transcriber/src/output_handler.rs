
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
        let mut inputs: [INPUT; 2] = unsafe { std::mem::zeroed() };

        for (i, input) in inputs.iter_mut().enumerate() {
            input.type_ = INPUT_KEYBOARD;
            unsafe {
                *input.u.ki_mut() = KEYBDINPUT {
                    wVk: 0,
                    wScan: c as u16,
                    dwFlags: 0x0004 | if i == 1 { 0x0002 } else { 0 }, // KEYEVENTF_UNICODE | (KEYEVENTF_KEYUP for second input)
                    time: 0,
                    dwExtraInfo: 0,
                };
            }
        }

        let result = unsafe {
            SendInput(2, inputs.as_mut_ptr(), std::mem::size_of::<INPUT>() as i32)
        };
        println!("SendInput result: {}", result);

        // Add a small delay after sending the input
        thread::sleep(time::Duration::from_millis(10));

        println!("Character '{}' should have been typed", c);
    }
}
