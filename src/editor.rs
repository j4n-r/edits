use crate::terminal;

pub fn process_key() -> Result<(), std::io::Error> {
    loop {
        terminal::refresh_screen();
        let c = terminal::read_key()?;
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

fn is_ctrl(c: u8) -> bool {
    c < 32 || c == 127
}
const fn ctrl_key(k: u8) -> u8 {
    k & 0x1f
}
