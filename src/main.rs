
fn main() {
    let devices = rtxflash::list_devices();
    println!("{:?}", devices);
}
