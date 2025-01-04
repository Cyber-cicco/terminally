use std::io::{stdout, Cursor, Stdout, Write};
mod insertion;
use insertion::handle_enter_key;

use crossterm::event::{read, Event::Key, KeyCode};
use crossterm::style::Print;
use crossterm::ExecutableCommand;
use crossterm::{cursor, terminal};

pub struct Editor {
    stdout: Stdout,
    mode: EditorMod,
    buffers: Vec<Buffer>,
}

pub struct Buffer {
    size_x: u16,
    size_y: u16,
    characters: Vec<char>,
    line_breaks_pos: Vec<u64>,
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
            buffers: Vec::new(),
        }
    }

    pub fn render_all(&mut self) {

    }

    pub fn handle_normal(&mut self) {
        loop {
            match read() {
                Ok(Key(event)) => match event.code {
                    KeyCode::Char(c) => {
                        if c == ':' {
                            self.mode = EditorMod::COMMAND;
                            break;
                        }
                        self.stdout
                            .execute(Print(c))
                            .unwrap();
                    }
                    KeyCode::Backspace => {
                        self.stdout
                            .execute(Print("I'm a fucking backspace !!!"))
                            .unwrap();
                    }
                    KeyCode::Enter => {
                        handle_enter_key(self);
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
        let (x, y) = terminal::size().unwrap();

        self.buffers.push(Buffer {
            size_x: x,
            size_y: y,
            characters: Vec::new(),
            line_breaks_pos: Vec::new(),
        });

        self.stdout.execute(terminal::EnterAlternateScreen).unwrap();
        terminal::enable_raw_mode().unwrap();

        for i in 0..y {
            self.stdout
                .execute(cursor::MoveTo(0, i))
                .unwrap();
            self.stdout
                .execute(Print("~"))
                .unwrap();
        }

        self.stdout.execute(cursor::MoveTo(1, 0)).unwrap();

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
