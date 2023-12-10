extern crate winapi;

use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;
use winapi::um::winuser::GetAsyncKeyState;

fn get_key_char(key_code: i32) -> Option<String> {
    match key_code {
        0x41..=0x5A => Some(((key_code + 0x20) as u8 as char).to_string()), // A-Z to a-z
        0x30..=0x39 | 0x60..=0x69 => Some((key_code as u8 as char).to_string()), // 0-9 and numpad
        0x20 => Some("Space".to_string()), // Space bar
        _ => None,
    }
}

fn main() {
    loop {
        for key_code in 0x08..=0xFE {
            unsafe {
                if GetAsyncKeyState(key_code) & 0x8000u16 as i16 != 0 {
                    if let Some(key_char) = get_key_char(key_code) {
                        println!("Key {} is pressed", key_char);
                        io::stdout().flush().unwrap();
                        sleep(Duration::from_millis(100));
                    }
                }
            }
        }
    }
}
