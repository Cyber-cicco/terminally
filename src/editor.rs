use crossterm::event::{read, Event::Key, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    mode : EditorMod
}

pub enum EditorMod {
    NORMAL,
    INSERTION,
    VISUAL,
    COMMAND
}

impl Editor {
    pub fn default() -> Self {
        Editor{
            mode: EditorMod::NORMAL
        }
    }
    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    match event.code {
                        KeyCode::Char(c) => {
                            print!("{c}");
                            if c == 'q' {
                                break;
                            }
                        }
                        KeyCode::Backspace => {
                            println!("I'm a fucking backspace !!!");
                        }
                        _ => ()
                    }
                }
                Err(err) => println!("Error reading input : {err}"),
                _ => ()
            }
        }
        disable_raw_mode().unwrap();
    }
}
