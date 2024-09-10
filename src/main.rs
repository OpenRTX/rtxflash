use std::env;
use std::fmt;
use std::io::{Error, ErrorKind};
use std::process;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter;
use text_colorizer::*; // 0.17.1

mod a36plus;

/// Supported targets
#[derive(Debug, EnumIter)]
enum Target {
    MD3X0,
    MDUV3X0,
    MD9600,
    GD77,
    DM1801,
    MOD17,
    TTWRPLUS,
    A36PLUS,
}

impl TryFrom<&str> for Target {
    type Error = ();

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        let v = v.to_uppercase();
        let v = str::replace(&v, "-", "");
        match v {
            x if x == "MD3X0" => Ok(Target::MD3X0),
            x if x == "MDUV3X0" => Ok(Target::MDUV3X0),
            x if x == "MD9600" => Ok(Target::MD9600),
            x if x == "GD77" => Ok(Target::GD77),
            x if x == "DM1801" => Ok(Target::DM1801),
            x if x == "MOD17" => Ok(Target::MOD17),
            x if x == "TTWRPLUS" => Ok(Target::TTWRPLUS),
            x if x == "A36PLUS" => Ok(Target::A36PLUS),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Target::MD3X0 => write!(f, "MD-3x0"),
            Target::MDUV3X0 => write!(f, "MD-UV3x0"),
            Target::MD9600 => write!(f, "MD-3x0"),
            Target::GD77 => write!(f, "GD77"),
            Target::DM1801 => write!(f, "DM1801"),
            Target::MOD17 => write!(f, "Mod17"),
            Target::TTWRPLUS => write!(f, "T-TWRPlus"),
            Target::A36PLUS => write!(f, "A36Plus"),
        }
    }
}

/// Print usage information of this tool
fn print_usage(cmd: &String) {
    eprintln!("{}: OpenRTX Flashing Module", "rtxflash".yellow());
    eprintln!("{}: invalid parameters", "Error".red().bold());
    eprintln!("Usage: {cmd} COMMAND [PARAM_0..PARAM_N]");
    eprintln!("commands:");
    eprintln!(" list                             Print available radios");
    eprintln!(" targets                          Print available targets");
    eprintln!(" wrap TARGET INPUT OUTPUT         Wrap selected file");
    eprintln!(" flash TARGET RADIO_ID FILE       Flash file on radio");
    process::exit(1);
}

fn wrap(target: Target, input_path: &str, output_path: &str) -> Result<(), Error> {
    match target {
        Target::MD3X0 => {}
        Target::MDUV3X0 => {}
        Target::MD9600 => {}
        Target::GD77 => {}
        Target::DM1801 => {}
        Target::MOD17 => {}
        Target::TTWRPLUS => {}
        Target::A36PLUS => a36plus::wrap(input_path, output_path)?,
    };
    Ok(())
}

fn flash(target: Target, radio_id: &str, file: &str) -> Result<(), Error> {
    match target {
        Target::MD3X0 => {}
        Target::MDUV3X0 => {}
        Target::MD9600 => {}
        Target::GD77 => {}
        Target::DM1801 => {}
        Target::MOD17 => {}
        Target::TTWRPLUS => {}
        Target::A36PLUS => a36plus::flash(radio_id, file)?,
    };
    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    // Print usage information
    if args.len() < 2 {
        print_usage(&args[0]);
    }

    let command = &args[1];
    let target_str = env::args().nth(2);

    match &command as &str {
        "list" => {
            let devices = rtxflash::get_devices();
            println!("{:?}", devices);
        }
        "targets" => {
            for target in Target::iter() {
                println!("{}", target);
            }
        }
        "wrap" => {
            if target_str.is_none() {
                print_usage(&args[0]);
            }
            let input_str = env::args().nth(3);
            let output_str = env::args().nth(4);
            if target_str.is_none() || input_str.is_none() || output_str.is_none() {
                print_usage(&args[0]);
            }
            let target =
                Target::try_from(target_str.unwrap().as_str()).expect("Target not recognized!");
            let input = input_str.unwrap();
            let output = output_str.unwrap();
            wrap(target, &input, &output)?;
        }
        "flash" => {
            let radio_str = env::args().nth(3);
            let file_str = env::args().nth(4);
            if target_str.is_none() || radio_str.is_none() || file_str.is_none() {
                print_usage(&args[0]);
            }
            let target =
                Target::try_from(target_str.unwrap().as_str()).expect("Target not recognized!");
            let radio = radio_str.unwrap();
            let file = file_str.unwrap();
            flash(target, &radio, &file)?;
        }
        _ => print_usage(&args[0]),
    };
    Ok(())
}
