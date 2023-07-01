use std::io::{stdin, stdout, Write};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;

use crossterm::event::{read, DisableFocusChange, EnableFocusChange};
use crossterm::execute;

fn print_events() -> crossterm::Result<()> {
    loop {
        read()?;
    }
}

fn main() {
    println!("^[[O => Focus out");
    println!("^[[I => Focus out");
    println!("Press ENTER to exit safely");
    println!("Read ANSI sequences: ");
    execute!(stdout(), EnableFocusChange).unwrap();
    MouseTerminal::from(stdout().into_raw_mode().unwrap())
        .flush()
        .unwrap();
    std::thread::spawn(move || print_events());

    stdin().read_line(&mut String::new()).unwrap();
    execute!(stdout(), DisableFocusChange).unwrap();
}
