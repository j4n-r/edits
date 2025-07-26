use std::{io::Read, sync::LazyLock};

static EDITOR_CONFIG: std::sync::LazyLock<EditorConfig> = LazyLock::new(|| EditorConfig {
    orig_termios: enable_raw_mode().expect("Enabling raw mode failed"),
    window: get_window_size().expect("Error getting window size"),
});

fn main() -> std::io::Result<()> {
    println!("{}", *EDITOR_CONFIG);
    process_key()?;
    Ok(())
}

fn read_key() -> Result<u8, std::io::Error> {
    let mut buffer: [u8; 1] = [0; 1];
    std::io::stdin()
        .read_exact(&mut buffer)
        .expect("read failed");
    Ok(buffer[0])
}

fn process_key() -> Result<(), std::io::Error> {
    loop {
        refresh_screen();
        let c = read_key()?;
        if c == ctrl_key(b'q') {
            print!("\x1b[2J");
            print!("\x1b[H");
            return Ok(());
        }
        if is_ctrl(c) {
            print!("{}\r\n", c);
        } else {
            print!("{}\r\n", c as char);
        }
    }
}

fn refresh_screen() {
    draw_rows();
    print!("\x1b[2J");
    print!("\x1b[H");
}

fn enable_raw_mode() -> Result<libc::termios, std::io::Error> {
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

fn disable_raw_mode(termios: libc::termios) -> Result<(), std::io::Error> {
    unsafe {
        if libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &termios) != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
}
fn is_ctrl(c: u8) -> bool {
    c < 32 || c == 127
}
const fn ctrl_key(k: u8) -> u8 {
    k & 0x1f
}

fn get_window_size() -> Result<libc::winsize, std::io::Error> {
    unsafe {
        let ws: libc::winsize = std::mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &ws) == -1 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(ws)
    }
}

struct EditorConfig {
    orig_termios: libc::termios,
    window: libc::winsize,
}

impl Drop for EditorConfig {
    fn drop(&mut self) {
        let _ = disable_raw_mode(self.orig_termios).expect("disabling raw mode failed");
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
