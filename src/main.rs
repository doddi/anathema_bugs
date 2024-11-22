mod index;

use anathema::{
    prelude::{Document, TuiBackend},
    runtime::Runtime,
};

fn main() {
    let tui = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .expect("");

    let mut runtime_builder = Runtime::builder(Document::new("@index"), tui);
    index::register(&mut runtime_builder);
    let mut runtime = runtime_builder.finish().expect("");
    runtime.run();
}
