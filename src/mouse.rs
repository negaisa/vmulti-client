use crate::device::{find_device, Device, DeviceError, CONTROL_REPORT_ID};
use std::mem::size_of;
use winapi::ctypes::c_void;

const MOUSE_REPORT_ID: u8 = 0x03;
const MOUSE_REPORT_SIZE: u8 = size_of::<MouseReport>() as u8;

pub struct Mouse {
    device: Device,
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

impl Mouse {
    pub fn init() -> Result<Self, DeviceError> {
        let device = find_device()?;

        Ok(Mouse { device })
    }

    pub fn send_click(
        &self,
        button: Option<MouseButton>,
        x: u16,
        y: u16,
        wheel_position: u8,
    ) -> bool {
        let button_id = match button {
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
            x,
            y,
            wheel_position,
        };

        return self
            .device
            .send_report(&mut report as *mut _ as *mut c_void);
    }
}
