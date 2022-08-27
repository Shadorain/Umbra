use umbra::{IEvent, Umbra};

/// Will be a simple example of rendering a Document with Umbra
fn main() {
    // let mut umbra = Umbra::new().config(serialized_data);

    let mut umbra: Umbra = Umbra::new().expect("Umbra error: {0}");
    umbra.init().expect("Panic... umbra init failed");

    loop {
        match umbra.read_event().unwrap() {
            IEvent::Key(key) => print!("Key {:?}", key),
            IEvent::Paste(s) => print!("Paste {0}", s),
            IEvent::FocusGained => print!("Window gained focus"),
            IEvent::FocusLost => print!("Window lost focus"),
        }
    }
}
