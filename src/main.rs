
fn main() {
    let devices = rtxflash::get_devices();
    println!("{:?}", devices);
    rtxflash::get_device_info(0);
}
