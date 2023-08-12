use crate::radio_tool_ffi::CxxDeviceInfo;

#[cxx::bridge(namespace = "radio_tool::radio")]
mod radio_tool_ffi {
    // Shared struct equivalent to RadioInfo
    #[derive(Debug)]
    struct CxxDeviceInfo {
        index: u16,
        manufacturer: String,
        model: String,
        port: String,
    }

    unsafe extern "C++" {
        include!("rtxflash/include/radio_tool.h");
        fn get_devices() -> Vec<CxxDeviceInfo>;
        fn get_device_info(index: u16);
        fn flash_device(index: u16, firmware_path: &str);
    }
}

pub fn get_devices() -> Vec<CxxDeviceInfo> {
    radio_tool_ffi::get_devices()
}

pub fn get_device_info(index: u16) {
    radio_tool_ffi::get_device_info(index)
}

pub fn flash_device(index: u16, firmware_path: &str) {
    radio_tool_ffi::flash_device(index, firmware_path)
}
