use umbra::{IEvent, Umbra, UError, Key, KeyModifiers};

struct Document {
    title: &'static str,
}

impl Document {
    fn new(title: &'static str) -> Self { Self { title } }
}

/// Will be a simple example of rendering a Document with Umbra
fn main() -> Result<(), UError> {
    let mut umbra: Umbra = Umbra::new()?;//.config(serialized_data);
    let document = Document::new("Document #1");

    umbra.set_title(document.title)?;

    // Main executive loop
    loop {
        match umbra.read_event()? {
            Some(event) => match event {
                // Quit on `C-q` keypress
                IEvent::Key(modif, key) => if key == Key::Char('q') && modif == KeyModifiers::CONTROL { break; },
                IEvent::Mouse(m) => println!("Mouse ptr: ({0}, {1})\r", m.x, m.y),
                IEvent::Resize(r) => println!("Screen has been resized to: ({0}, {1})\r", r.x, r.y),
                IEvent::Paste(s) => print!("Paste {0}\r", s),
                IEvent::FocusGained => print!("Window gained focus\r"),
                IEvent::FocusLost => print!("Window lost focus\r"),
            },
            None => (),
        }
    }

    Ok(())
}
