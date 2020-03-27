use crate::device::{find_device, Device, DeviceError, CONTROL_REPORT_ID};
use crate::display::DisplayInfo;
use std::mem::size_of;
use winapi::ctypes::c_void;

const MOUSE_REPORT_ID: u8 = 0x03;
const MOUSE_REPORT_SIZE: u8 = size_of::<MouseReport>() as u8;

pub struct MouseClick {
    button: Option<MouseButton>,
    x: u16,
    y: u16,
    wheel_position: u8,
    display_index: u8,
}

impl MouseClick {
    pub fn new() -> Self {
        MouseClick {
            button: None,
            x: 0,
            y: 0,
            wheel_position: 0,
            display_index: 0,
        }
    }

    pub fn set_position(mut self, x: u16, y: u16) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn set_button(mut self, button: MouseButton) -> Self {
        self.button = Some(button);
        self
    }

    pub fn set_wheel_position(mut self, wheel_position: u8) -> Self {
        self.wheel_position = wheel_position;
        self
    }

    pub fn set_display_index(mut self, display_index: u8) -> Self {
        self.display_index = display_index;
        self
    }
}

pub enum MouseButton {
    LeftButton,
    RightButton,
    MiddleButton,
}

#[repr(C)]
#[derive(Debug)]
struct MouseReport {
    control_report_id: u8,
    report_length: u8,
    report_id: u8,
    button: u8,
    x: u16,
    y: u16,
    wheel_position: u8,
}

pub struct Mouse {
    device: Device,
    displays_info: Vec<DisplayInfo>,
}

impl Mouse {
    pub fn init(displays_info: Vec<DisplayInfo>) -> Result<Self, DeviceError> {
        let device = find_device()?;

        Ok(Mouse {
            device,
            displays_info,
        })
    }

    pub fn send_click(&self, click: MouseClick) -> bool {
        let button_id = match click.button {
            None => 0,
            Some(MouseButton::LeftButton) => 1,
            Some(MouseButton::RightButton) => 2,
            Some(MouseButton::MiddleButton) => 3,
        };

        let mut report = MouseReport {
            control_report_id: CONTROL_REPORT_ID,
            report_length: MOUSE_REPORT_SIZE,
            report_id: MOUSE_REPORT_ID,
            button: button_id,
            x: click.x,
            y: click.y,
            wheel_position: click.wheel_position,
        };

        return self
            .device
            .send_report(&mut report as *mut _ as *mut c_void);
    }
}
