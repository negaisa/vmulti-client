use std::mem::size_of;
use std::ptr::null_mut;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::LPARAM;
use winapi::shared::windef::HDC;
use winapi::shared::windef::HMONITOR;
use winapi::shared::windef::LPRECT;
use winapi::shared::windef::RECT;
use winapi::um::winuser::MONITORINFO;
use winapi::um::winuser::{EnumDisplayMonitors, GetMonitorInfoW};

#[derive(Debug)]
pub struct DisplayInfo {
    pub primary: bool,
    pub position: DisplayPosition,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug)]
pub struct DisplayPosition {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

pub fn get_displays_info() -> Vec<DisplayInfo> {
    let mut displays_info: Vec<DisplayInfo> = Vec::new();

    unsafe {
        EnumDisplayMonitors(
            null_mut(),
            null_mut(),
            Some(display_info_callback),
            &mut displays_info as *mut Vec<DisplayInfo> as _,
        );
    }

    return displays_info;
}

unsafe extern "system" fn display_info_callback(
    monitor: HMONITOR,
    _hdc: HDC,
    rect_ptr: LPRECT,
    dw_data: LPARAM,
) -> BOOL {
    let displays_info = &mut *(dw_data as *mut Vec<DisplayInfo>);
    let rect = *rect_ptr;

    let mut monitor_info = MONITORINFO {
        cbSize: 0,
        rcMonitor: RECT {
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        },
        rcWork: RECT {
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        },
        dwFlags: 0,
    };

    monitor_info.cbSize = size_of::<MONITORINFO>() as u32;

    GetMonitorInfoW(monitor, &mut monitor_info);

    let display_position = DisplayPosition {
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
    };

    let display_info = DisplayInfo {
        primary: monitor_info.dwFlags == 1,
        position: display_position,
        width: (rect.left - rect.right).abs() as u16,
        height: ((rect.top - rect.bottom).abs()) as u16,
    };

    displays_info.push(display_info);

    1
}
