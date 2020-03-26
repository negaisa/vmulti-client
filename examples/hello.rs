use std::thread;
use std::time::Duration;
use vmulti_client::keyboard::{Keyboard, KeyboardKey, KeyboardModifierKey, KeysClick};

fn main() {
    let keyboard = Keyboard::init().unwrap();

    keyboard.send_click(KeysClick::new().set_modifier(KeyboardModifierKey::RightWindows));
    keyboard.send_click(KeysClick::new());

    let sleep_duration = Duration::from_millis(100);
    thread::sleep(sleep_duration);

    keyboard.send_click(
        KeysClick::new()
            .set_modifier(KeyboardModifierKey::RightShift)
            .set_key(KeyboardKey::KeyH),
    );

    keyboard.send_click(KeysClick::new().set_key(KeyboardKey::KeyE));

    keyboard.send_click(KeysClick::new().set_key(KeyboardKey::KeyL));
    keyboard.send_click(KeysClick::new());

    keyboard.send_click(KeysClick::new().set_key(KeyboardKey::KeyL));

    keyboard.send_click(KeysClick::new().set_key(KeyboardKey::KeyO));
    keyboard.send_click(KeysClick::new());
}
