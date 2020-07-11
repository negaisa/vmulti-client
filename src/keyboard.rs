use crate::device::{find_device, Device, DeviceError, CONTROL_REPORT_ID};
use itertools::concat;
use itertools::join;
use std::convert::TryFrom;
use std::fmt::Display;
use std::mem::size_of;
use winapi::_core::fmt::{Error, Formatter};
use winapi::ctypes::c_void;

const KEYBOARD_REPORT_ID: u8 = 0x07;
const KEYBOARD_REPORT_SIZE: u8 = size_of::<KeyboardReport>() as u8;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum KeyboardKey {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Enter,
    Esc,
    Del,
    Tab,
    Space,
}

impl KeyboardKey {
    // Key codes may be found at table 12.
    // https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf
    fn id(&self) -> u8 {
        return match &self {
            KeyboardKey::A => 0x04,
            KeyboardKey::B => 0x05,
            KeyboardKey::C => 0x06,
            KeyboardKey::D => 0x07,
            KeyboardKey::E => 0x08,
            KeyboardKey::F => 0x09,
            KeyboardKey::G => 0x0A,
            KeyboardKey::H => 0x0B,
            KeyboardKey::I => 0x0C,
            KeyboardKey::J => 0x0D,
            KeyboardKey::K => 0x0E,
            KeyboardKey::L => 0x0F,
            KeyboardKey::M => 0x10,
            KeyboardKey::N => 0x11,
            KeyboardKey::O => 0x12,
            KeyboardKey::P => 0x13,
            KeyboardKey::Q => 0x14,
            KeyboardKey::R => 0x15,
            KeyboardKey::S => 0x16,
            KeyboardKey::T => 0x17,
            KeyboardKey::U => 0x18,
            KeyboardKey::V => 0x19,
            KeyboardKey::W => 0x1A,
            KeyboardKey::X => 0x1B,
            KeyboardKey::Y => 0x1C,
            KeyboardKey::Z => 0x1D,
            KeyboardKey::Num1 => 0x1E,
            KeyboardKey::Num2 => 0x1F,
            KeyboardKey::Num3 => 0x20,
            KeyboardKey::Num4 => 0x21,
            KeyboardKey::Num5 => 0x22,
            KeyboardKey::Num6 => 0x23,
            KeyboardKey::Num7 => 0x24,
            KeyboardKey::Num8 => 0x25,
            KeyboardKey::Num9 => 0x26,
            KeyboardKey::Num0 => 0x27,
            KeyboardKey::Enter => 0x28,
            KeyboardKey::Esc => 0x29,
            KeyboardKey::Del => 0x2A,
            KeyboardKey::Tab => 0x2B,
            KeyboardKey::Space => 0x2C,
        };
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
    fn mask(&self) -> u8 {
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct KeysClick {
    modifiers: Vec<KeyboardModifierKey>,
    keys: Vec<KeyboardKey>,
}

impl KeysClick {
    pub fn new(key: KeyboardKey) -> Self {
        KeysClick::empty().add_key(key)
    }

    pub fn empty() -> Self {
        KeysClick {
            modifiers: Vec::new(),
            keys: Vec::with_capacity(6),
        }
    }

    pub fn add_modifier(mut self, modifier: KeyboardModifierKey) -> Self {
        self.modifiers.push(modifier);
        self
    }

    pub fn add_key(mut self, key: KeyboardKey) -> Self {
        self.keys.push(key);
        self
    }
}

impl TryFrom<&str> for KeyboardKey {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "a" => Ok(KeyboardKey::A),
            "b" => Ok(KeyboardKey::B),
            "c" => Ok(KeyboardKey::C),
            "d" => Ok(KeyboardKey::D),
            "e" => Ok(KeyboardKey::E),
            "f" => Ok(KeyboardKey::F),
            "g" => Ok(KeyboardKey::G),
            "h" => Ok(KeyboardKey::H),
            "i" => Ok(KeyboardKey::I),
            "j" => Ok(KeyboardKey::J),
            "k" => Ok(KeyboardKey::K),
            "l" => Ok(KeyboardKey::L),
            "m" => Ok(KeyboardKey::M),
            "n" => Ok(KeyboardKey::N),
            "o" => Ok(KeyboardKey::O),
            "p" => Ok(KeyboardKey::P),
            "q" => Ok(KeyboardKey::Q),
            "r" => Ok(KeyboardKey::R),
            "s" => Ok(KeyboardKey::S),
            "t" => Ok(KeyboardKey::T),
            "u" => Ok(KeyboardKey::U),
            "v" => Ok(KeyboardKey::V),
            "w" => Ok(KeyboardKey::W),
            "x" => Ok(KeyboardKey::X),
            "y" => Ok(KeyboardKey::Y),
            "z" => Ok(KeyboardKey::Z),
            "1" => Ok(KeyboardKey::Num1),
            "2" => Ok(KeyboardKey::Num2),
            "3" => Ok(KeyboardKey::Num3),
            "4" => Ok(KeyboardKey::Num4),
            "5" => Ok(KeyboardKey::Num5),
            "6" => Ok(KeyboardKey::Num6),
            "7" => Ok(KeyboardKey::Num7),
            "8" => Ok(KeyboardKey::Num8),
            "9" => Ok(KeyboardKey::Num9),
            "0" => Ok(KeyboardKey::Num0),
            "enter" => Ok(KeyboardKey::Enter),
            "esc" => Ok(KeyboardKey::Esc),
            "del" => Ok(KeyboardKey::Del),
            "tab" => Ok(KeyboardKey::Tab),
            "space" => Ok(KeyboardKey::Space),
            v if v.is_empty() => Err("Input is empty"),
            _ => Err("Invalid input"),
        }
    }
}

impl TryFrom<&str> for KeyboardModifierKey {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "left-ctrl" => Ok(KeyboardModifierKey::LeftControl),
            "left-shift" => Ok(KeyboardModifierKey::LeftShift),
            "left-alt" => Ok(KeyboardModifierKey::LeftAlt),
            "left-win" => Ok(KeyboardModifierKey::LeftWindows),
            "right-ctrl" => Ok(KeyboardModifierKey::RightControl),
            "right-shift" => Ok(KeyboardModifierKey::RightShift),
            "right-alt" => Ok(KeyboardModifierKey::RightAlt),
            "right-win" => Ok(KeyboardModifierKey::RightWindows),
            v if v.is_empty() => Err("Input is empty"),
            _ => Err("Invalid input"),
        }
    }
}

impl TryFrom<&str> for KeysClick {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("Input is empty");
        }

        let split: Vec<&str> = value.split("+").collect();
        let mut keys_click = KeysClick::empty();

        for arg in split {
            let try_modifier = KeyboardModifierKey::try_from(arg);
            let try_key = KeyboardKey::try_from(arg);

            keys_click = match (try_modifier, try_key) {
                (Ok(modifier), Err(_)) => keys_click.add_modifier(modifier),
                (Err(_), Ok(key)) => keys_click.add_key(key),
                _ => return Err("Invalid input"),
            }
        }

        Ok(keys_click)
    }
}

impl Display for KeyboardKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let value = match self {
            KeyboardKey::A => "a",
            KeyboardKey::B => "b",
            KeyboardKey::C => "c",
            KeyboardKey::D => "d",
            KeyboardKey::E => "e",
            KeyboardKey::F => "f",
            KeyboardKey::G => "g",
            KeyboardKey::H => "h",
            KeyboardKey::I => "i",
            KeyboardKey::J => "j",
            KeyboardKey::K => "k",
            KeyboardKey::L => "l",
            KeyboardKey::M => "m",
            KeyboardKey::N => "n",
            KeyboardKey::O => "o",
            KeyboardKey::P => "p",
            KeyboardKey::Q => "q",
            KeyboardKey::R => "r",
            KeyboardKey::S => "s",
            KeyboardKey::T => "t",
            KeyboardKey::U => "u",
            KeyboardKey::V => "v",
            KeyboardKey::W => "w",
            KeyboardKey::X => "x",
            KeyboardKey::Y => "y",
            KeyboardKey::Z => "z",
            KeyboardKey::Num1 => "1",
            KeyboardKey::Num2 => "2",
            KeyboardKey::Num3 => "3",
            KeyboardKey::Num4 => "4",
            KeyboardKey::Num5 => "5",
            KeyboardKey::Num6 => "6",
            KeyboardKey::Num7 => "7",
            KeyboardKey::Num8 => "8",
            KeyboardKey::Num9 => "9",
            KeyboardKey::Num0 => "0",
            KeyboardKey::Enter => "enter",
            KeyboardKey::Esc => "esc",
            KeyboardKey::Del => "del",
            KeyboardKey::Tab => "tab",
            KeyboardKey::Space => "space",
        };

        write!(f, "{}", value)
    }
}

impl Display for KeyboardModifierKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let value = match self {
            KeyboardModifierKey::LeftControl => "left-ctrl",
            KeyboardModifierKey::LeftShift => "left-shift",
            KeyboardModifierKey::LeftAlt => "left-alt",
            KeyboardModifierKey::LeftWindows => "left-win",
            KeyboardModifierKey::RightControl => "right-ctrl",
            KeyboardModifierKey::RightShift => "right-shift",
            KeyboardModifierKey::RightAlt => "right-alt",
            KeyboardModifierKey::RightWindows => "right-win",
        };

        write!(f, "{}", value)
    }
}

impl Display for KeysClick {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let modifiers_string: Vec<String> = self.modifiers.iter().map(|m| m.to_string()).collect();
        let keys_string: Vec<String> = self.keys.iter().map(|k| k.to_string()).collect();
        let concat = concat(vec![modifiers_string, keys_string]);

        write!(f, "{}", join(&concat, "+"))
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
        let mut keys = [0; 6];

        for i in 0..6 {
            if let Some(key) = keys_click.keys.get(i) {
                keys[i] = key.id();
            }
        }

        let modifiers = keys_click.modifiers.iter().fold(0, |acc, m| acc | m.mask());

        let mut report = KeyboardReport {
            control_report_id: CONTROL_REPORT_ID,
            report_length: KEYBOARD_REPORT_SIZE,
            report_id: KEYBOARD_REPORT_ID,
            modifiers,
            _reserved: 0,
            keys,
        };

        return self
            .device
            .send_report(&mut report as *mut _ as *mut c_void);
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        self.send_click(KeysClick::empty());
    }
}
