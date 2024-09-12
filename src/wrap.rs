use std::io::Error;

use crate::a36plus;
use crate::target;

pub fn wrap(target: target::Target, input_path: &str, output_path: &str) -> Result<(), Error> {
    match target {
        target::Target::MD3X0 => {}
        target::Target::MDUV3X0 => {}
        target::Target::MD9600 => {}
        target::Target::GD77 => {}
        target::Target::DM1801 => {}
        target::Target::MOD17 => {}
        target::Target::TTWRPLUS => {}
        target::Target::A36PLUS => a36plus::wrap(input_path, output_path)?,
    };
    Ok(())
}
