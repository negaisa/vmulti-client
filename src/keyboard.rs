use crate::device::{find_device, Device, DeviceError, CONTROL_REPORT_ID};
use std::mem::size_of;
use winapi::ctypes::c_void;

const KEYBOARD_REPORT_ID: u8 = 0x07;
const KEYBOARD_REPORT_SIZE: u8 = size_of::<KeyboardReport>() as u8;

pub enum KeyboardKey {
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    KeyEnter,
    KeyEsc,
    KeyDel,
    KeyTab,
    KeySpace,
}

impl KeyboardKey {
    // Key codes may be found at table 12.
    // https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf
    fn usage_id(&self) -> u8 {
        return match &self {
            KeyboardKey::KeyA => 0x04,
            KeyboardKey::KeyB => 0x05,
            KeyboardKey::KeyC => 0x06,
            KeyboardKey::KeyD => 0x07,
            KeyboardKey::KeyE => 0x08,
            KeyboardKey::KeyF => 0x09,
            KeyboardKey::KeyG => 0x0A,
            KeyboardKey::KeyH => 0x0B,
            KeyboardKey::KeyI => 0x0C,
            KeyboardKey::KeyJ => 0x0D,
            KeyboardKey::KeyK => 0x0E,
            KeyboardKey::KeyL => 0x0F,
            KeyboardKey::KeyM => 0x10,
            KeyboardKey::KeyN => 0x11,
            KeyboardKey::KeyO => 0x12,
            KeyboardKey::KeyP => 0x13,
            KeyboardKey::KeyQ => 0x14,
            KeyboardKey::KeyR => 0x15,
            KeyboardKey::KeyS => 0x16,
            KeyboardKey::KeyT => 0x17,
            KeyboardKey::KeyU => 0x18,
            KeyboardKey::KeyV => 0x19,
            KeyboardKey::KeyW => 0x1A,
            KeyboardKey::KeyX => 0x1B,
            KeyboardKey::KeyY => 0x1C,
            KeyboardKey::KeyZ => 0x1D,
            KeyboardKey::Key1 => 0x1E,
            KeyboardKey::Key2 => 0x1F,
            KeyboardKey::Key3 => 0x20,
            KeyboardKey::Key4 => 0x21,
            KeyboardKey::Key5 => 0x22,
            KeyboardKey::Key6 => 0x23,
            KeyboardKey::Key7 => 0x24,
            KeyboardKey::Key8 => 0x25,
            KeyboardKey::Key9 => 0x26,
            KeyboardKey::Key0 => 0x27,
            KeyboardKey::KeyEnter => 0x28,
            KeyboardKey::KeyEsc => 0x29,
            KeyboardKey::KeyDel => 0x2A,
            KeyboardKey::KeyTab => 0x2B,
            KeyboardKey::KeySpace => 0x2C,
        };
    }
}

pub enum KeyboardModifierKey {
    LeftControl,
    LeftShift,
    LeftAlt,
    LeftWindows,
    RightControl,
    RightShift,
    RightAlt,
    RightWindows,
}

impl KeyboardModifierKey {
    fn usage_id(&self) -> u8 {
        return match &self {
            KeyboardModifierKey::LeftControl => 1,
            KeyboardModifierKey::LeftShift => 2,
            KeyboardModifierKey::LeftAlt => 4,
            KeyboardModifierKey::LeftWindows => 8,
            KeyboardModifierKey::RightControl => 16,
            KeyboardModifierKey::RightShift => 32,
            KeyboardModifierKey::RightAlt => 64,
            KeyboardModifierKey::RightWindows => 128,
        };
    }
}

pub struct KeysClick {
    modifiers: u8,
    current_keys_index: usize,
    keys: [u8; 6],
}

impl KeysClick {
    pub fn new() -> Self {
        KeysClick {
            modifiers: 0,
            current_keys_index: 0,
            keys: [0, 0, 0, 0, 0, 0],
        }
    }

    pub fn set_modifier(mut self, modifier: KeyboardModifierKey) -> Self {
        self.modifiers |= modifier.usage_id();
        self
    }

    pub fn set_key(mut self, key: KeyboardKey) -> Self {
        if self.keys.len() > self.current_keys_index + 1 {
            self.keys[self.current_keys_index] = key.usage_id();
            self.current_keys_index += 1;
        }

        self
    }
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

pub struct Keyboard {
    device: Device,
}

impl Keyboard {
    pub fn init() -> Result<Self, DeviceError> {
        let device = find_device()?;

        Ok(Keyboard { device })
    }

    pub fn send_click(&self, keys_click: KeysClick) -> bool {
        let mut report = KeyboardReport {
            control_report_id: CONTROL_REPORT_ID,
            report_length: KEYBOARD_REPORT_SIZE,
            report_id: KEYBOARD_REPORT_ID,
            modifiers: keys_click.modifiers,
            _reserved: 0,
            keys: keys_click.keys,
        };

        return self
            .device
            .send_report(&mut report as *mut _ as *mut c_void);
    }
}
