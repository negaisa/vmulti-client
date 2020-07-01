use std::thread;
use std::time::Duration;
use vmulti_client::display::get_displays_info;
use vmulti_client::mouse::{Mouse, MouseButton, MouseClick};

fn main() {
    let displays_info = get_displays_info();
    let mouse = Mouse::init(&displays_info).unwrap();

    let display_info = &displays_info[1];
    let center_x = display_info.width / 2;
    let center_y = display_info.height / 2;

    mouse.send_click(
        MouseClick::new()
            .set_display_index(1)
            .set_position(center_x, center_y),
    );

    for _ in 0..1000 {
        mouse.send_click(
            MouseClick::new()
                .set_display_index(1)
                .set_button(MouseButton::Right)
                .set_position(center_x - 3, center_y),
        );

        thread::sleep(Duration::from_millis(10));

        let mouse_position = mouse.get_mouse_position();

        println!(
            "Current mouse position: X: {}, Y: {}",
            mouse_position.x, mouse_position.y
        );
    }

    mouse.send_click(
        MouseClick::new()
            .set_display_index(1)
            .set_position(center_x, center_y),
    );
}
