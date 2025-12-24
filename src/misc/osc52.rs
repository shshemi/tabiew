use std::{
    io::Write,
    sync::{LazyLock, Mutex},
};

use base64::Engine;

static OSC52_BUFFER: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::default()));

pub fn flush_osc52_buffer() {
    let mut buffer = OSC52_BUFFER.lock().unwrap();
    if !buffer.is_empty() {
        for seq in buffer.drain(..) {
            print!("{}", seq);
        }
        std::io::stdout().flush().unwrap();
    }
}

pub trait CopyToClipboardOsc52 {
    fn copy_to_clipboard_via_osc52(&self);
}

impl<T> CopyToClipboardOsc52 for T
where
    T: AsRef<[u8]>,
{
    fn copy_to_clipboard_via_osc52(&self) {
        let encoded = base64::engine::general_purpose::STANDARD.encode(self);
        let sequence = format!("\x1b]52;c;{encoded}\x07");
        let mut buffer = OSC52_BUFFER.lock().unwrap();
        buffer.clear();
        buffer.push_str(&sequence);
    }
}
