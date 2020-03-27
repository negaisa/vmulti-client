use std::thread;
use std::time::Duration;
use vmulti_client::mouse::{Mouse, MouseButton, MouseClick};

fn main() {
    let mouse = Mouse::init().unwrap();

    // click start menu.
    mouse.send_click(
        MouseClick::new()
            .set_button(MouseButton::LeftButton)
            .set_position(1000, 32767),
    );

    mouse.send_click(MouseClick::new().set_position(1000, 32767));

    let sleep_duration = Duration::from_millis(1000);
    thread::sleep(sleep_duration);

    // click shutdown menu.
    mouse.send_click(
        MouseClick::new()
            .set_button(MouseButton::LeftButton)
            .set_position(1000, 30767),
    );
    mouse.send_click(MouseClick::new().set_position(1000, 30767));

    thread::sleep(sleep_duration);
    mouse.send_click(MouseClick::new().set_position(1000, 26767));
}
