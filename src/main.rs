use std::io::Read;

fn main() -> std::io::Result<()> {
    let _ = Cleanup {
        orig_termios: enable_raw_mode().expect("enabling raw mode failed"),
    };
    enable_raw_mode()?;
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
        let c = read_key()?;
        if is_ctrl(c) {
            print!("{}\r\n", c);
        } else {
            print!("{}\r\n", c as char);
        }
        if c == ctrl_key(b'q') {
            std::process::exit(1);
        }
    }
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

struct Cleanup {
    orig_termios: libc::termios,
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        let _ = disable_raw_mode(self.orig_termios).expect("disabling raw mode failed");
    }
}
