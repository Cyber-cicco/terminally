use std::io::Stdout;

use crossterm::{cursor, ExecutableCommand};

use super::Buffer;

fn move_up(buffer: &mut Buffer, stdout: &mut Stdout) -> Result<(), std::io::Error> {
    assert!(buffer.active);
    assert!(buffer.on_screen);
    let (pos_x, pos_y) = cursor::position()?;
    stdout.execute(cursor::MoveTo())?;
    Ok(())
}
