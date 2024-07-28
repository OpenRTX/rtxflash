
fn main() {
    let devices = rtxflash::get_devices();
    println!("{:?}", devices);
    rtxflash::get_device_info(0);
    rtxflash::flash_device(&devices[0], "~/Downloads/openrtx_mduv3x0_wrap")
}
