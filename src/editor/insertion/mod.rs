use crossterm::{cursor, style::Print, ExecutableCommand};

use super::Editor;

pub fn handle_enter_key(editor: &mut Editor) -> Result<(), std::io::Error> {
    editor.stdout.execute(Print("\n"))?;
    let (_, pos_y) = cursor::position()?;
    editor.stdout.execute(cursor::MoveTo(0, pos_y))?;
    Ok(())
}
