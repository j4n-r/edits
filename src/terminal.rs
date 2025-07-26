use crate::editor;
use crate::EDITOR_CONFIG;
use editor::Cursor;
use std::io::{Read, Write};

pub fn enable_raw_mode() -> Result<libc::termios, std::io::Error> {
    unsafe {
        let mut termios: libc::termios = std::mem::zeroed();
        if libc::tcgetattr(libc::STDIN_FILENO, &mut termios) != 0 {
            return Err(std::io::Error::last_os_error());
        }
        let orig_termios = termios.clone();

        termios.c_iflag &= !(libc::ICRNL | libc::IXON | libc::BRKINT | libc::ISTRIP | libc::INPCK);
        termios.c_oflag &= !(libc::OPOST);
        termios.c_cflag |= libc::CS8;
        termios.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);

        if libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut termios) != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(orig_termios)
    }
}

pub fn disable_raw_mode(termios: libc::termios) -> Result<(), std::io::Error> {
    unsafe {
        if libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &termios) != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
}
pub fn read_key() -> Result<u8, std::io::Error> {
    let mut buffer: [u8; 1] = [0; 1];
    std::io::stdin()
        .read_exact(&mut buffer)
        .expect("read failed");
    Ok(buffer[0])
}

pub fn refresh_screen(cursor: &Cursor) {
    let mut term_buf: Vec<String> = Vec::with_capacity(1000);
    term_buf.push("\x1b[2J".to_string()); // clear screen
    term_buf.push("\x1b[H".to_string()); // move to 1:1
    draw_rows(&mut term_buf);
    let cursor_pos = format!("\x1b[{};{}H", cursor.row, cursor.col);
    term_buf.push(cursor_pos); // move to row:col
    print!("{}", term_buf.join(""));
    std::io::stdout().flush().unwrap();
}

fn draw_rows(term_buf: &mut Vec<String>) {
    for _i in 0..EDITOR_CONFIG.window.ws_row - 1 {
        term_buf.push("~\r\n".to_string())
    }
    term_buf.push("~".to_string());
}

pub fn get_window_size() -> Result<libc::winsize, std::io::Error> {
    unsafe {
        let ws: libc::winsize = std::mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &ws) == -1 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(ws)
    }
}
