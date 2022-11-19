use std::ops::Range;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use rand::Rng;

pub fn copy_to_clipboard(text: String) {
    let mut clipboard = ClipboardContext::new().expect("Failed to get clipboard context");
    clipboard
        .set_contents(text)
        .expect("Failed to set clipboard contents");
}

pub fn random_string(rng: i32) -> String {
    Range {start: 0, end: rng}
        .map(|_| {
            let mut rng = rand::thread_rng();
            let random_char: char = rng.gen_range('a'..='z');
            random_char
        })
        .collect()
}
