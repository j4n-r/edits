use std::sync::{LazyLock, Mutex};
mod editor;
mod terminal;

pub static EDITOR_CONFIG: LazyLock<EditorConfig> = LazyLock::new(|| EditorConfig {
    orig_termios: terminal::enable_raw_mode().expect("Enabling raw mode failed"),
    window: terminal::get_window_size().expect("Error getting window size"),
});

fn main() -> std::io::Result<()> {
    println!("{}", *EDITOR_CONFIG);
    editor::process_key()?;
    Ok(())
}

pub struct EditorConfig {
    orig_termios: libc::termios,
    window: libc::winsize,
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
