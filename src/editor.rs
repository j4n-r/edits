use crate::log;
use std::io::{BufRead, Read};

use crate::{terminal, EDITOR_CONFIG};

#[derive(Debug)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Buffer {
    pub lines: Vec<String>,
}

#[derive(Debug)]
pub struct Debug {
    pub keycode: usize,
}

pub fn process_key(buf: &mut Buffer) -> Result<(), std::io::Error> {
    let mut vis_cursor = Cursor { row: 1, col: 2 };
    let mut debug_info = Debug { keycode: 0 };
    loop {
        terminal::refresh_screen(&vis_cursor, buf, &debug_info);
        let c = terminal::read_key()?;
        log::debug(c.to_string());
        debug_info.keycode = c as usize;
        match c {
            127 => {
                delete(1, &vis_cursor, buf);
                if vis_cursor.col > 2 {
                    vis_cursor.col -= 1
                }
            }
            c if c == ctrl_key(b'h') => {
                if vis_cursor.col > 2 {
                    vis_cursor.col -= 1
                }
            }
            c if c == ctrl_key(b'j') => vis_cursor.row += 1,
            c if c == ctrl_key(b'k') => {
                if vis_cursor.row > 1 {
                    vis_cursor.row -= 1
                }
            }
            c if c == ctrl_key(b'l') => vis_cursor.col += 1,
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

fn delete(num_to_delete: usize, vis_cursor: &Cursor, buf: &mut Buffer) {
    let mem_cursor = vis_to_mem_cursor(vis_cursor, buf);
    let row = buf
        .lines
        .get(mem_cursor.row - 1)
        .expect("row not found in delete()");
    let start = &row[0..mem_cursor.col - 2 - num_to_delete];
    let end = &row[mem_cursor.col - 2..];
    let mut new_line = String::from(start);
    new_line.push_str(end);
    let curr_line = buf
        .lines
        .get_mut(mem_cursor.row - 1)
        .expect("row not found");
    *curr_line = new_line;
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

fn vis_to_mem_cursor(vis_cursor: &Cursor, buf: &Buffer) -> Cursor {
    let line = &buf.lines[vis_cursor.row - 1];
    let tabs_before_cursor = line
        .chars()
        .take(vis_cursor.col as usize)
        .filter(|&c| c == '\t')
        .count();
    let mem_col = vis_cursor.col + (tabs_before_cursor * (EDITOR_CONFIG.tab_stop_size - 1));
    Cursor {
        row: vis_cursor.row,
        col: mem_col,
    }
}
