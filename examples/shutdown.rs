use std::thread;
use std::time::Duration;
use vmulti_client::mouse::{Mouse, MouseButton};

fn main() {
    let mouse = Mouse::init().unwrap();

    // click start menu.
    mouse.send_click(Some(MouseButton::LeftButton), 1000, 32767, 0);
    mouse.send_click(None, 1000, 32767, 0);

    let sleep_duration = Duration::from_millis(1000);
    thread::sleep(sleep_duration);

    // click shutdown menu.
    mouse.send_click(Some(MouseButton::LeftButton), 1000, 30767, 0);
    mouse.send_click(None, 1000, 30767, 0);

    thread::sleep(sleep_duration);

    // move to shutdown.
    mouse.send_click(None, 1000, 26767, 0);
}