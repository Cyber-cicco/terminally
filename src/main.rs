mod editor;
use editor::Editor;

fn main() {
    dbg!("\x1b[2J");
    let somesize = (10, 20);
    dbg!(somesize);
    dbg!(somesize.0);
    dbg!(somesize.1);
    for number in 0..10 {
        dbg!(number);
    }
    Editor::default().run();
}
