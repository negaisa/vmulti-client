use std::mem::MaybeUninit;
use std::ptr::null_mut;

use winapi::ctypes::c_void;
use winapi::shared::guiddef::GUID;
use winapi::shared::hidpi::{HidP_GetCaps, PHIDP_PREPARSED_DATA};
use winapi::shared::hidpi::{HIDP_CAPS, HIDP_STATUS_SUCCESS};
use winapi::shared::hidsdi::{HidD_GetAttributes, HidD_GetHidGuid};
use winapi::shared::hidsdi::{HidD_GetPreparsedData, HIDD_ATTRIBUTES};
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::{HANDLE, PVOID, ULONG};
use winapi::um::cfgmgr32::{
    CM_Get_Device_Interface_ListW, CM_Get_Device_Interface_List_SizeA,
    CM_GET_DEVICE_INTERFACE_LIST_PRESENT, CR_SUCCESS,
};
use winapi::um::fileapi::OPEN_EXISTING;
use winapi::um::fileapi::{CreateFileA, WriteFile};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winnt::FILE_SHARE_WRITE;
use winapi::um::winnt::GENERIC_WRITE;

const DEVICE_VENDOR_ID: u16 = 0x00FF;
const DEVICE_PRODUCT_ID: u16 = 0xBACC;
const DEVICE_VERSION_ID: u16 = 0x0001;
const DEVICE_USAGE_PAGE: u16 = 0xFF00;
const DEVICE_USAGE: u16 = 0x0001;

const CONTROL_REPORT_ID: u8 = 0x40;
const CONTROL_REPORT_SIZE: u32 = 0x41;
const KEYBOARD_REPORT_ID: u8 = 0x07;
const KEYBOARD_REPORT_SIZE: u8 = 0x09;

#[derive(Debug)]
pub enum DeviceError {
    FailedToGetDeviceInterfaceListSize,
    FailedToGetDeviceInterfaceList,
    EmptyDeviceInterfaceList,
    NoValidHandle,
    NotFound,
}

pub struct Device {
    handle: HANDLE,
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

impl Device {
    pub(crate) fn send_keyboard_report(&self, modifiers: u8, keys: [u8; 6]) -> bool {
        let mut report = KeyboardReport {
            control_report_id: CONTROL_REPORT_ID,
            report_length: KEYBOARD_REPORT_SIZE,
            report_id: KEYBOARD_REPORT_ID,
            modifiers,
            _reserved: 0,
            keys,
        };

        return self.send_report(&mut report as *mut _ as *mut c_void);
    }

    fn send_report(&self, data: PVOID) -> bool {
        let mut bytes_written = MaybeUninit::<DWORD>::uninit();

        unsafe {
            return WriteFile(
                self.handle,
                data,
                CONTROL_REPORT_SIZE,
                bytes_written.as_mut_ptr(),
                null_mut(),
            ) == 1;
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

pub(crate) fn find_device() -> Result<Device, DeviceError> {
    let mut maybe_guid = MaybeUninit::<GUID>::uninit();

    unsafe { HidD_GetHidGuid(maybe_guid.as_mut_ptr()) };

    let handle = unsafe { find_handle(maybe_guid.assume_init())? };

    Ok(Device { handle })
}

unsafe fn find_handle(mut guid: GUID) -> Result<HANDLE, DeviceError> {
    let mut maybe_device_interface_list_length = MaybeUninit::<ULONG>::uninit();

    let interface_list_size_result = CM_Get_Device_Interface_List_SizeA(
        maybe_device_interface_list_length.as_mut_ptr(),
        &mut guid as *mut _,
        null_mut(),
        CM_GET_DEVICE_INTERFACE_LIST_PRESENT,
    );

    if interface_list_size_result != CR_SUCCESS {
        return Err(DeviceError::FailedToGetDeviceInterfaceListSize);
    }

    let interface_list_length = maybe_device_interface_list_length.assume_init() as usize;

    // Will be 1 if no interfaces found.
    if interface_list_length <= 1 {
        return Err(DeviceError::EmptyDeviceInterfaceList);
    }

    let mut interface_list_vec = vec![0u16; interface_list_length as usize];

    let interface_list_result = CM_Get_Device_Interface_ListW(
        &mut guid as *mut _,
        null_mut(),
        interface_list_vec.as_mut_ptr(),
        interface_list_length as u32,
        CM_GET_DEVICE_INTERFACE_LIST_PRESENT as u32,
    );

    if interface_list_result != CR_SUCCESS {
        return Err(DeviceError::FailedToGetDeviceInterfaceList);
    }

    let interface_list_str = String::from_utf16(&interface_list_vec).unwrap();
    let mut has_valid_handle = false;

    for interface in interface_list_str.split("\0") {
        if interface.trim().is_empty() {
            continue;
        }

        let handle = CreateFileA(
            interface.as_ptr() as *const _,
            GENERIC_WRITE,
            FILE_SHARE_WRITE,
            null_mut(),
            OPEN_EXISTING,
            0,
            null_mut(),
        );

        if handle == INVALID_HANDLE_VALUE {
            continue;
        } else {
            has_valid_handle = true;
        }

        if is_our_device(handle) {
            return Ok(handle);
        }

        CloseHandle(handle);
    }

    if !has_valid_handle {
        return Err(DeviceError::NoValidHandle);
    }

    Err(DeviceError::NotFound)
}

unsafe fn is_our_device(handle: HANDLE) -> bool {
    let mut maybe_attributes = MaybeUninit::<HIDD_ATTRIBUTES>::uninit();

    if HidD_GetAttributes(handle, maybe_attributes.as_mut_ptr()) == 0 {
        return false;
    }

    let attributes = maybe_attributes.assume_init();

    if attributes.VendorID != DEVICE_VENDOR_ID {
        return false;
    }

    if attributes.ProductID != DEVICE_PRODUCT_ID {
        return false;
    }

    if attributes.VersionNumber != DEVICE_VERSION_ID {
        return false;
    }

    let mut maybe_ppd = MaybeUninit::<PHIDP_PREPARSED_DATA>::uninit();

    if HidD_GetPreparsedData(handle, maybe_ppd.as_mut_ptr()) == 0 {
        return false;
    }

    let mut maybe_caps = MaybeUninit::<HIDP_CAPS>::uninit();

    let get_caps_result = HidP_GetCaps(maybe_ppd.assume_init(), maybe_caps.as_mut_ptr());

    if get_caps_result != HIDP_STATUS_SUCCESS {
        return false;
    }

    let caps = maybe_caps.assume_init();

    if caps.UsagePage != DEVICE_USAGE_PAGE {
        return false;
    }

    if caps.Usage != DEVICE_USAGE {
        return false;
    }

    return true;
}
