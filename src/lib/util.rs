use std::io::Write;
use std::ops::Range;
use std::path::PathBuf;

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

pub fn ask_yes_no(question: &str) -> bool {
    let mut answer = String::new();
    print!("{} ", question);
    std::io::stdout().flush().expect("Failed to flush stdout");
    std::io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    answer.trim().to_lowercase() == "y"
}

pub fn get_notepad_or_gedit() -> String {
    let mut notepad = String::new();
    if cfg!(target_os = "windows") {
        notepad = "notepad".to_string();
    } else if cfg!(target_os = "linux") {
        notepad = "gedit".to_string();
    }
    notepad
}
pub fn open_file_in_editor(file: &PathBuf) {
    let editor = get_notepad_or_gedit();
    let mut command = std::process::Command::new(editor);
    command.arg(file);
    command.spawn().expect("Failed to spawn editor");
}

pub fn get_longest_string(strings: Vec<&str>) -> usize {
    let mut longest = 0;
    for string in strings {
        if string.len() > longest {
            longest = string.len();
        }
    }
    longest
}

pub fn calc_padding(longest: usize, string: &str) -> usize {
    longest - string.len()
}