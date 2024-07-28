use crate::radio_tool_ffi::DeviceInfo;

#[cxx::bridge(namespace = "radio_tool::radio")]
pub mod radio_tool_ffi {
    // Shared struct equivalent to RadioInfo
    #[derive(Clone, Debug)]
    pub struct DeviceInfo {
        index: u16,
        manufacturer: String,
        model: String,
        port: String,
    }

    unsafe extern "C++" {
        include!("rtxflash/include/radio_tool.h");
        fn get_devices() -> Vec<DeviceInfo>;
        fn get_device_info(index: u16);
        fn flash_device(index: u16, firmware_path: &str);
    }

}

impl std::fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: [{}] {} {}", self.index, self.port, self.manufacturer, self.model)
    }
}

pub fn get_devices() -> Vec<DeviceInfo> {
    radio_tool_ffi::get_devices()
}

pub fn get_device_info(index: u16) {
    radio_tool_ffi::get_device_info(index)
}

pub fn flash_device(device: &DeviceInfo, firmware_path: &str) {
    radio_tool_ffi::flash_device(device.index, firmware_path)
}
