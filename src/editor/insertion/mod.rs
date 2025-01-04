use crossterm::{cursor, style::Print, ExecutableCommand};

use super::Editor;

pub fn handle_enter_key(editor: &mut Editor) {
    editor.stdout.execute(Print("\n")).unwrap();
    let (_, posy) = cursor::position().unwrap();
    editor.stdout.execute(cursor::MoveTo(1, posy)).unwrap();
}
