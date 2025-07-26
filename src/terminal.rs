use crate::EDITOR_CONFIG;
use std::io::Read;

pub fn read_key() -> Result<u8, std::io::Error> {
    let mut buffer: [u8; 1] = [0; 1];
    std::io::stdin()
        .read_exact(&mut buffer)
        .expect("read failed");
    Ok(buffer[0])
}

pub fn refresh_screen() {
    draw_rows();
    print!("\x1b[2J");
    print!("\x1b[H");
}

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

fn draw_rows() {
    for _i in 0..EDITOR_CONFIG.window.ws_row - 1 {
        print!("~\r\n")
    }
    print!("~");
}

pub fn disable_raw_mode(termios: libc::termios) -> Result<(), std::io::Error> {
    unsafe {
        if libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &termios) != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
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
