use enigo::*;
use fltk::{app, button::Button, input::Input, prelude::*, window::Window};
use std::thread;
use std::time::Duration;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut window = Window::new(100, 100, 200, 200, "FORCE PASTE");
    let input = Input::new(10, 10, 100, 20, "");
    let mut button = Button::new(10, 40, 100, 20, "Paste");
    window.end();
    window.show();
    button.set_callback(move |_| {
        let input_text = input.value();
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        thread::sleep(Duration::from_secs(2));
        enigo.text(&input_text).expect("Failed to paste text");
    });
    app.run().unwrap();
}