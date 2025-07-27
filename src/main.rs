mod editor;
mod terminal;
use std::sync::LazyLock;

use editor::Buffer;

pub static EDITOR_CONFIG: LazyLock<EditorConfig> = LazyLock::new(|| EditorConfig {
    orig_termios: terminal::enable_raw_mode().expect("Enabling raw mode failed"),
    window: terminal::get_window_size().expect("Error getting window size"),
    tab_stop_size: 4,
    welcome_message: true,
});

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = std::env::args().collect();
    let file_path: String;
    let mut buf = Buffer { lines: Vec::new() };
    if args.len() > 1 {
        file_path = args[1].clone();
        buf = editor::load_file(file_path)?;
    }

    editor::process_key(&mut buf)?;
    Ok(())
}

pub struct EditorConfig {
    orig_termios: libc::termios,
    window: libc::winsize,
    tab_stop_size: usize,
    welcome_message: bool,
}

impl Drop for EditorConfig {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode(self.orig_termios).expect("disabling raw mode failed");
    }
}

impl std::fmt::Display for EditorConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Cols: {}, Rows: {})",
            self.window.ws_col, self.window.ws_row
        )
    }
}
