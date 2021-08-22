extern crate termion;

use termion::raw::IntoRawMode;

use super::Config;
use std::io::{stdin, stdout, Read, Write};

pub struct CliConfig {}

struct terminal {}

pub fn run(config: Config) {
    let stdout = stdout();
    // Go into raw mode for getting the character
    // input without newline and to not write the input
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    // Clear the screen first
    write!(
        stdout,
        "{}{}Stuff",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    );
    stdout.flush().unwrap();

    let mut bytes = stdin.bytes();

    loop {
        // For each character input
        let b = bytes.next().unwrap().unwrap();

        if config.local_echo {
            write!(stdout, "{}", b as char);
            stdout.flush().unwrap();
        }
    }
}
