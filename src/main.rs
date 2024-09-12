use std::env;
use std::io::Error;
use std::process;
use strum::IntoEnumIterator;
use text_colorizer::*; // 0.17.1

mod a36plus;
mod flash;
mod target;
mod wrap;

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
            let devices = target::get_devices();
            println!("{:?}", devices);
        }
        "targets" => {
            for target in target::Target::iter() {
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
            let target = target::Target::try_from(target_str.unwrap().as_str())
                .expect("target::Target not recognized!");
            let input = input_str.unwrap();
            let output = output_str.unwrap();
            wrap::wrap(target, &input, &output)?;
        }
        "flash" => {
            let radio_str = env::args().nth(3);
            let file_str = env::args().nth(4);
            if target_str.is_none() || radio_str.is_none() || file_str.is_none() {
                print_usage(&args[0]);
            }
            let target = target::Target::try_from(target_str.unwrap().as_str())
                .expect("target::Target not recognized!");
            let radio = radio_str.unwrap();
            let file = file_str.unwrap();
            flash::flash(target, &radio, &file)?;
        }
        _ => print_usage(&args[0]),
    };
    Ok(())
}
