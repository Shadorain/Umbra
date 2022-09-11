use umbra::{IEvent, Umbra, UError, Key, KeyModifiers};

// struct Document {
//     title: &str,
// }

/// Will be a simple example of rendering a Document with Umbra
fn main() -> Result<(), UError> {
    let mut umbra: Umbra = Umbra::new()?;//.config(serialized_data);

    loop {
        match umbra.read_event()? {
            IEvent::Key(modif, key) => if key == Key::Char('q') && modif == KeyModifiers::CONTROL { break; },
            IEvent::Mouse(m) => println!("Mouse ptr: ({0}, {1})", m.x, m.y),
            IEvent::Resize(r) => println!("Screen has been resized to: ({0}, {1})", r.x, r.y),
            IEvent::Paste(s) => print!("Paste {0}", s),
            IEvent::FocusGained => print!("Window gained focus"),
            IEvent::FocusLost => print!("Window lost focus"),
        }
    }

    Ok(())
}
