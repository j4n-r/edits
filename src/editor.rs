use crate::{terminal, EDITOR_CONFIG};

pub fn process_key() -> Result<(), std::io::Error> {
    let mut cursor = Cursor { row: 1, col: 1 };
    loop {
        terminal::refresh_screen(&cursor);
        let c = terminal::read_key()?;
        match c {
            c if c == ctrl_key(b'h') => {
                if cursor.col > 1 {
                    cursor.col -= 1
                }
            }
            c if c == ctrl_key(b'j') => cursor.row += 1,
            c if c == ctrl_key(b'k') => {
                if cursor.row > 1 {
                    cursor.row -= 1
                }
            }
            c if c == ctrl_key(b'l') => cursor.col += 1,
            c if c == ctrl_key(b'q') => {
                print!("\x1b[2J");
                print!("\x1b[H");
                print!("Exited by user");
                return Ok(());
            }
            _ => (),
        }
    }
}

fn is_ctrl(c: u8) -> bool {
    c < 32 || c == 127
}
const fn ctrl_key(k: u8) -> u8 {
    k & 0x1f
}

pub struct Cursor {
    pub row: u32,
    pub col: u32,
}
