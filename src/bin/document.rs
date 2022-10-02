use umbra::{IEvent, Umbra, UError, Key, KeyModifiers};

struct Document {
    title: &'static str,
}

impl Document {
    fn new(title: &'static str) -> Self { Self { title } }
}

/// A simple example of rendering a Document with Umbra
fn main() -> Result<(), UError> {
    let mut umbra: Umbra = Umbra::new()?;//.config(serialized_data);
    let document = Document::new("Document #1");

    umbra.set_title(document.title)?;

    // Main executive loop
    loop {
        if let Some(event) = umbra.read_input()? {
            match event {
                // Quit on `C-q` keypress
                IEvent::Key(modif, key) => if key == Key::Char('q') && modif == KeyModifiers::CONTROL { break; },
                IEvent::Mouse(m) => println!("Mouse ptr: ({0}, {1})\r", m.x, m.y),
                IEvent::Resize(r) => println!("Screen has been resized to: ({0}, {1})\r", r.x, r.y),
                IEvent::Paste(s) => println!("Paste {0}\r", s),
                IEvent::FocusGained => println!("Window gained focus\r"),
                IEvent::FocusLost => println!("Window lost focus\r"),
            }
        }
    }

    Ok(())
}
