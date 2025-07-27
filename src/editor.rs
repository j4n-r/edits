use std::io::{BufRead, Read};

use crate::{terminal, EDITOR_CONFIG};

pub struct VisCursor {
    pub row: usize,
    pub col: usize,
}

pub struct MemCursor {
    pub row: usize,
    pub col: usize,
}

pub struct Buffer {
    pub lines: Vec<String>,
}

pub fn process_key(buf: &mut Buffer) -> Result<(), std::io::Error> {
    let mut cursor = VisCursor { row: 1, col: 1 };
    loop {
        terminal::refresh_screen(&cursor, buf);
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

pub fn load_file(file_path: String) -> Result<Buffer, anyhow::Error> {
    let file = std::fs::File::open(file_path)?;
    let meta = file.metadata()?;
    let reader = std::io::BufReader::new(file);
    let buf_len = usize::try_from(meta.len())?;
    let mut buf = Buffer {
        lines: Vec::with_capacity(buf_len),
    };
    for line in reader.lines().map_while(Result::ok) {
        buf.lines.push(line);
    }
    Ok(buf)
}

fn is_ctrl(c: u8) -> bool {
    c < 32 || c == 127
}
const fn ctrl_key(k: u8) -> u8 {
    k & 0x1f
}

fn vis_to_mem_cursor(mut vis_cursor: VisCursor, buf: &Buffer) {
    let line = &buf.lines[vis_cursor.row - 1];
    let tabs_before_cursor = line
        .chars()
        .take(vis_cursor.col as usize)
        .filter(|&c| c == '\t')
        .count();
    vis_cursor.col += tabs_before_cursor * (EDITOR_CONFIG.tab_stop_size - 1);
}
