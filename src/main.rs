use std::io::{stdout, BufWriter, Stdout};

use crossterm::{event as e, queue, style as s, terminal as t, Result};

pub struct Terminal {
    buffer: BufWriter<Stdout>,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            buffer: BufWriter::new(stdout()),
        }
    }
    fn init(&mut self) -> Result<()> {
        queue!(
            self.buffer,
            t::EnterAlternateScreen,
            s::ResetColor,
            t::Clear(t::ClearType::All),
            e::EnableMouseCapture,
            e::EnableFocusChange,
        )?;
        t::enable_raw_mode()?;
        Ok(())
    }
}

fn main() {
    while true {
        Terminal::new().init();
    }
}
