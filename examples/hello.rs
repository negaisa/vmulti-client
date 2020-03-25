use std::thread;
use std::time::Duration;
use vmulti_client::keyboard::Keyboard;

fn main() {
    let keyboard = Keyboard::init().unwrap();

    // Windows key.
    keyboard.send_click(8, [0, 0, 0, 0, 0, 0]);
    keyboard.send_click(0, [0, 0, 0, 0, 0, 0]);

    let sleep_duration = Duration::from_millis(100);
    thread::sleep(sleep_duration);

    // H
    keyboard.send_click(2, [0x0b, 0, 0, 0, 0, 0]);
    // e
    keyboard.send_click(0, [0x08, 0, 0, 0, 0, 0]);
    // l
    keyboard.send_click(0, [0x0f, 0, 0, 0, 0, 0]);
    keyboard.send_click(0, [0x0, 0, 0, 0, 0, 0]);
    // l
    keyboard.send_click(0, [0x0f, 0, 0, 0, 0, 0]);
    // o
    keyboard.send_click(0, [0x12, 0, 0, 0, 0, 0]);
    keyboard.send_click(0, [0x0, 0, 0, 0, 0, 0]);
}