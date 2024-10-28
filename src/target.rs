use std::fmt;
use strum_macros::EnumIter;

/// Supported targets
#[derive(Debug, EnumIter)]
pub enum Target {
    MD3X0,
    MDUV3X0,
    MD9600,
    GD77,
    DM1801,
    MOD17,
    TTWRPLUS,
    A36PLUS,
}

#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub index: u16,
    pub manufacturer: String,
    pub model: String,
    pub port: String,
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}] ({}) {} {}",
            self.index, self.port, self.manufacturer, self.model
        )
    }
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

pub fn get_devices() -> Vec<DeviceInfo> {
    let mut targets = vec![] as Vec<DeviceInfo>;

    // TODO: build a list of OpenRTX radios VID:PID
    // let devices = usb_enumeration::enumerate();
    // println!("{:#?}", devices);

    // Add serial port based devices
    for port in serialport::available_ports().unwrap() {
        let device = DeviceInfo {
            index: targets.len() as u16,
            manufacturer: String::from("Unknown"),
            model: String::from("Unknown"),
            port: port.port_name,
        };
        targets.push(device);
    }
    targets
}
