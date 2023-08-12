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
        pub fn get_devices() -> Vec<CxxDeviceInfo>;
        fn flash_device() -> Result<()>;
    }
}

pub fn get_devices() -> Vec<CxxDeviceInfo> {
    radio_tool_ffi::get_devices()
}

pub fn install() {
    println!("Flashing OpenRTX firmware");
    if let Err(err) = radio_tool_ffi::flash_device() {
        eprintln!("Error: {}", err);
        // process::exit(1);
    }
    println!("Firmware flash completed");
    println!("Please reboot the device");
}
