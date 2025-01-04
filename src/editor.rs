use std::io::{self, Write};

use crossterm::event::{read, Event::Key, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    mode: EditorMod,
}

pub enum EditorMod {
    NORMAL,
    INSERTION,
    VISUAL,
    COMMAND,
    QUIT,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            mode: EditorMod::NORMAL,
        }
    }

    pub fn handle_normal(&mut self) {
        loop {
            match read() {
                Ok(Key(event)) => match event.code {
                    KeyCode::Char(c) => {
                        print!("{c}");
                        if c == ':' {
                            self.mode = EditorMod::COMMAND;
                            break;
                        }
                    }
                    KeyCode::Backspace => {
                        println!("I'm a fucking backspace !!!");
                    }
                    _ => (),
                },
                Err(err) => println!("Error reading input : {err}"),
                _ => (),
            }
        }
    }
    pub fn handle_insertion(&self) {}
    pub fn handle_visual(&self) {}
    pub fn handle_command(&mut self) {
        loop {
            match read() {
                Ok(Key(event)) => match event.code {
                    KeyCode::Char(c) => {
                        print!("{c}");
                        if c == 'q' {
                            self.mode = EditorMod::QUIT;
                            break;
                        }
                    }
                    _ => (),
                },
                Err(err) => println!("Error reading input : {err}"),
                _ => (),
            }
        }
    }

    pub fn run(&mut self) {
        enable_raw_mode().unwrap();
        loop {
            match &self.mode {
                EditorMod::NORMAL => self.handle_normal(),
                EditorMod::INSERTION => self.handle_insertion(),
                EditorMod::VISUAL => self.handle_visual(),
                EditorMod::COMMAND => self.handle_command(),
                EditorMod::QUIT => break,
            }
        }
        disable_raw_mode().unwrap();
    }
}
