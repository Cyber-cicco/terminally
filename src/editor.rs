use std::io::{stdout, Stdout, Write};

use crossterm::event::{read, Event::Key, KeyCode};
use crossterm::style::Print;
use crossterm::{cursor, terminal};
use crossterm::ExecutableCommand;

pub struct Editor {
    stdout: Stdout,
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
            stdout: stdout(),
            mode: EditorMod::NORMAL,
        }
    }

    pub fn handle_normal(&mut self) {
        loop {
            match read() {
                Ok(Key(event)) => match event.code {
                    KeyCode::Char(c) => {
                        self.stdout.execute(Print(c)).unwrap();
                        if c == ':' {
                            self.mode = EditorMod::COMMAND;
                            break;
                        }
                    }
                    KeyCode::Backspace => {
                        self.stdout.execute(Print("I'm a fucking backspace !!!")).unwrap();
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
        self.stdout.execute(terminal::EnterAlternateScreen).unwrap();
        self.stdout.execute(cursor::MoveTo(0,0)).unwrap();
        terminal::enable_raw_mode().unwrap();
        // let (x, y) = terminal::size().unwrap();
        loop {
            match &self.mode {
                EditorMod::NORMAL => self.handle_normal(),
                EditorMod::INSERTION => self.handle_insertion(),
                EditorMod::VISUAL => self.handle_visual(),
                EditorMod::COMMAND => self.handle_command(),
                EditorMod::QUIT => break,
            }
        }
        terminal::disable_raw_mode().unwrap();
    }
}
