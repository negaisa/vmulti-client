use crate::device::{find_device, Device, DeviceError, CONTROL_REPORT_ID};
use winapi::ctypes::c_void;
use std::mem::size_of;

const KEYBOARD_REPORT_ID: u8 = 0x07;
const KEYBOARD_REPORT_SIZE: u8 = size_of::<KeyboardReport>() as u8;

pub struct Keyboard {
    device: Device,
}

#[repr(C)]
#[derive(Debug)]
struct KeyboardReport {
    control_report_id: u8,
    report_length: u8,
    report_id: u8,
    modifiers: u8,
    _reserved: u8,
    keys: [u8; 6],
}

impl Keyboard {
    pub fn init() -> Result<Self, DeviceError> {
        let device = find_device()?;

        Ok(Keyboard { device })
    }

    pub fn send_click(&self, modifiers: u8, keys: [u8; 6]) -> bool {
        let mut report = KeyboardReport {
            control_report_id: CONTROL_REPORT_ID,
            report_length: KEYBOARD_REPORT_SIZE,
            report_id: KEYBOARD_REPORT_ID,
            modifiers,
            _reserved: 0,
            keys,
        };

        return self.device.send_report(&mut report as *mut _ as *mut c_void);
    }

}
