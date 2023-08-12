
fn main() {
    let devices = rtxflash::get_devices();
    println!("{:?}", devices);
    rtxflash::get_device_info(0);
    rtxflash::flash_device(0, "~/Downloads/openrtx_mduv3x0_wrap")
}
