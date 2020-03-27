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
    };

    displays_info.push(display_info);

    1
}

#[derive(Debug)]
pub struct DisplayInfo {
    primary: bool,
    position: DisplayPosition,
}

#[derive(Debug)]
pub(crate) struct DisplayPosition {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
