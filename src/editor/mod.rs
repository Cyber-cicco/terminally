mod insertion;
mod movements;
use std::io::{stdout, Stdout};
use std::usize;
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

    cursor_pos: u64,
    active: bool,

    on_screen: bool,

    characters: Vec<char>,
    line_breaks_pos: Vec<u64>,
}

impl Buffer {
    pub fn get_cursor_line(&self) -> usize {
        let end = self.line_breaks_pos.len() as usize;

        // If there is no line break, you are on line 1
        if end == 0 {
            return 1;
        }

        let mut curr_pos = end / 2;
        let mut prev_pos = end - 1;
        let mut prev_positions_vec_lesser = Vec::new();
        prev_positions_vec_lesser.push(self.line_breaks_pos[prev_pos] < self.cursor_pos);
        loop {
            //TODO : finir ça avec une feuille blanche à coté
            if self.cursor_pos > self.line_breaks_pos[curr_pos] {
                prev_positions_vec_lesser.push(false);
                let old_pos = curr_pos;
                curr_pos = (curr_pos + prev_pos) / 2;
                prev_pos = curr_pos;
                continue;
            }
            if self.cursor_pos < self.line_breaks_pos[curr_pos] {
                prev_positions_vec_lesser.push(false);
                break;
            }
        }
        return curr_pos + 1;
    }
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

    fn handle_normal(&mut self) -> Result<(), std::io::Error> {
        loop {
            match read() {
                Ok(Key(event)) => match event.code {
                    KeyCode::Char(c) => {
                        if c == ':' {
                            self.mode = EditorMod::COMMAND;
                            break;
                        }
                        self.stdout.execute(Print(c))?;
                    }
                    KeyCode::Backspace => {
                        self.stdout.execute(Print("I'm a fucking backspace !!!"))?;
                    }
                    KeyCode::Enter => {
                        handle_enter_key(self)?;
                    }
                    _ => (),
                },
                Err(err) => println!("Error reading input : {err}"),
                _ => (),
            }
        }
        Ok(())
    }

    fn handle_insertion(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn handle_visual(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn handle_command(&mut self) -> Result<(), std::io::Error> {
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
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let (x, y) = terminal::size().unwrap();

        self.buffers.push(Buffer {
            size_x: x,
            size_y: y,
            characters: Vec::new(),
            line_breaks_pos: Vec::new(),
            cursor_pos: 0,
            active: true,
            on_screen: true,
        });

        self.stdout.execute(terminal::EnterAlternateScreen).unwrap();
        terminal::enable_raw_mode().unwrap();

        for i in 0..y {
            self.stdout.execute(cursor::MoveTo(0, i))?;
            self.stdout.execute(Print("~"))?;
        }

        self.stdout.execute(cursor::MoveTo(1, 0)).unwrap();

        loop {
            match &self.mode {
                EditorMod::NORMAL => self.handle_normal()?,
                EditorMod::INSERTION => self.handle_insertion()?,
                EditorMod::VISUAL => self.handle_visual()?,
                EditorMod::COMMAND => self.handle_command()?,
                EditorMod::QUIT => break,
            }
        }
        terminal::disable_raw_mode().unwrap();
        Ok(())
    }
}
