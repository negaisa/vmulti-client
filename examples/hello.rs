use std::thread;
use std::time::Duration;
use vmulti_client::keyboard::{Keyboard, KeyboardKey, KeyboardModifierKey, KeysClick};

fn main() {
    let keyboard = Keyboard::init().unwrap();

    keyboard.send_click(KeysClick::empty().add_modifier(KeyboardModifierKey::RightWindows));
    keyboard.send_click(KeysClick::empty());

    let sleep_duration = Duration::from_millis(100);
    thread::sleep(sleep_duration);

    keyboard.send_click(
        KeysClick::new(KeyboardKey::H).add_modifier(KeyboardModifierKey::RightShift),
    );

    keyboard.send_click(KeysClick::new(KeyboardKey::E));

    keyboard.send_click(KeysClick::new(KeyboardKey::L));
    keyboard.send_click(KeysClick::empty());

    keyboard.send_click(KeysClick::new(KeyboardKey::L));

    keyboard.send_click(KeysClick::new(KeyboardKey::O));
    keyboard.send_click(KeysClick::empty());
}
