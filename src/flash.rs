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
    match target {
        target::Target::MD3X0 => {}
        target::Target::MDUV3X0 => {}
        target::Target::MD9600 => {}
        target::Target::GD77 => {}
        target::Target::DM1801 => {}
        target::Target::MOD17 => {}
        target::Target::TTWRPLUS => {}
        target::Target::A36PLUS => a36plus::flash(radio_id, file, progress)?,
    };
    Ok(())
}
