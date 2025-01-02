use std::io::Error;
use std::sync::mpsc::Sender;

use crate::a36plus;
use crate::target;

pub fn flash(
    target: target::Target,
    radio_id: String,
    file: String,
    progress: Option<&Sender<(usize, usize)>>,
) -> Result<(), Error> {
    let res = match target {
        target::Target::MD3X0 => Ok(()),
        target::Target::MDUV3X0 => Ok(()),
        target::Target::MD9600 => Ok(()),
        target::Target::GD77 => Ok(()),
        target::Target::DM1801 => Ok(()),
        target::Target::MOD17 => Ok(()),
        target::Target::TTWRPLUS => Ok(()),
        target::Target::A36PLUS => a36plus::flash(radio_id, file, progress),
    };

    // Report back error condition using progress channel
    if res.is_err() {
        if progress.is_some() {
            match progress.unwrap().send((0, 0)) {
                Err(e) => println!("Error when logging progress: {e}"),
                Ok(_) => (),
            }
        }
    }
    res
}
