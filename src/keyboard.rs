use crate::device::{find_device, Device, DeviceError};

pub struct Keyboard {
    device: Device,
}

impl Keyboard {
    pub fn init() -> Result<Self, DeviceError> {
        let device = find_device()?;

        Ok(Keyboard { device })
    }

    pub fn send_click(&self, modifiers: u8, keys: [u8; 6]) -> bool {
        self.device.send_keyboard_report(modifiers, keys)
    }
}
