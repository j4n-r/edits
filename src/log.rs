use std::io::Write;

fn error(msg: String) {}

pub fn debug(msg: String) {
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("editor.log")
    {
        let debug_msg = format!("DEBUG: {}\n", msg);
        file.write_all(debug_msg.as_bytes())
            .expect("writing debug message failed");
    }
}
