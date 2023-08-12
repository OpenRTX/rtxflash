#[cxx::bridge(namespace = "radio_tool::radio")]
mod ffi {
    unsafe extern "C++" {
        include!("rtxflash/include/radio_tool.h");
        fn list_devices();
        fn flash_radio() -> Result<()>;
    }
}

pub fn list_devices() {
    ffi::list_devices();
}

pub fn install() {
    println!("Flashing OpenRTX firmware");
    if let Err(err) = ffi::flash_radio() {
        eprintln!("Error: {}", err);
        // process::exit(1);
    }
    println!("Firmware flash completed");
    println!("Please reboot the radio");
}
