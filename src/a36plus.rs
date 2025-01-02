use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::mpsc::Sender;
use std::time::Duration;

const PACKAGE_SIZE: usize = 1024; // 1KB
const XOR_KEY1: &[u8; 4] = b"KDHT";
const XOR_KEY2: &[u8; 4] = b"RBGI";

/*
 * 0x52 ... (Read Data): This command is handled in FUN_08007ec8 and FUN_08009a8c. The address to read from is specified in bytes 1 and 2. The number of bytes to read is specified in byte 3. If the address is 0x8000, VFO settings are read. If the address is 0x9000, global settings are read. Otherwise, data is read directly from the SPI flash. The read data is then sent back over the UART.
 * 0x57 ... (Write Data): Also handled in FUN_08007ec8 and FUN_08009a8c. The address to write to is specified in bytes 1 and 2. The number of bytes to write is specified in byte 3. Data to be written follows in subsequent bytes. If the address is 0x8000, VFO settings are written. If the address is 0x9000, global settings are written. Otherwise, data is written directly to the SPI flash.
 */

enum BootloaderCmd {
    HELLO = 0x01,    // Begin communication
    TRANSFER = 0x03, // Send firmware
    SIZE = 0x04,     // Send firmware transfer size
    REBOOT = 0x45,   // Reboot the radio
}

// enum FirmwareCmd {
//     HANDSHAKE = 0x02,
//     SETADDRESS = 0x03,
//     ERASE = 0x04,
//     OVER = 0x06,
//     D = 0x44,       // Sent during PROGRAM
//     UNKNOWN = 0x46, // The function of this command is unknown
//     READ = 0x52,
//     U = 0x55,       // Sent after PROGRAM and MODEL
//     WRITE = 0x57,
// }

// This looks like CRC16 CCITT
fn crc16(data: &[u8], offset: usize, count: usize) -> u16 {
    let mut num1: u16 = 0;
    for index1 in 0..count {
        let num2: u16 = data[index1 + offset] as u16;
        num1 ^= num2 << 8;
        for _ in 0..8 {
            if (num1 & 0x8000) == 0x8000 {
                num1 = num1 << 1 ^ 0x1021;
            } else {
                num1 <<= 1;
            }
        }
    }
    return num1;
}

// This function encodes the message to be sent using the radio bootloader serial protocol
fn fw_cmd(cmd: u8, args: u8, input: &[u8], size: usize) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0; 5 + input.len() + 3];
    buffer[0] = 0xaa;
    buffer[1] = cmd;
    buffer[2] = args;
    buffer[3] = (size as u16 >> 8) as u8;
    buffer[4] = size as u8;
    buffer[5..5 + input.len()].copy_from_slice(input);
    let digest = crc16(&buffer, 1, input.len() + 4);
    buffer[5 + input.len()] = (digest >> 8) as u8;
    buffer[5 + input.len() + 1] = digest as u8;
    buffer[5 + input.len() + 2] = 0xef;
    return buffer;
}

// // This function encodes the message to be sent using the fw serial protocol for data write
// fn data_cmd(cmd: u8, package_id: u16, data: &[u8], size: usize) -> Vec<u8> {
//     let mut buffer: Vec<u8> = vec![0; 5 + data.len() + 3];
//     buffer[0] = 0xa5;
//     buffer[1] = cmd;
//     buffer[2] = (package_id >> 8) as u8;
//     buffer[3] = package_id as u8;
//     buffer[4] = ((size as u16) >> 8) as u8;
//     buffer[5] = size as u8;
//     buffer[6..6 + data.len()].copy_from_slice(data);
//     let digest = crc16(&buffer, 1, data.len() + 4);
//     buffer[6 + data.len()] = (digest >> 8) as u8;
//     buffer[6 + data.len() + 1] = digest as u8;
//     buffer[6 + data.len() + 2] = 0xef;
//     return buffer;
// }

pub fn hello(port: &str) -> Result<(), Error> {
    let mut rx_buffer: Vec<u8> = vec![0; 256];
    let mut n_read: usize = 0;

    // Open a new serial port instance
    let mut serial_port = serialport::new(port, 115_200)
        .timeout(Duration::from_millis(1000))
        .open()?;

    // Discard any data in the serial port buffer
    _ = serial_port.read(&mut rx_buffer);

    let data = "BOOTLOADER";
    let tx_buffer = fw_cmd(
        BootloaderCmd::HELLO as u8,
        0,
        data.as_bytes(),
        data.as_bytes().len(),
    );
    _ = serial_port.write(&tx_buffer);

    while n_read < 3 {
        n_read = n_read + serial_port.read(&mut rx_buffer[n_read..]).unwrap();
    }

    if rx_buffer[0] != 0xaa || rx_buffer[1] != BootloaderCmd::HELLO as u8 || rx_buffer[2] != 0x06 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("[hello] Bad response from bootloader: {:?}", rx_buffer),
        ));
    } else {
        Ok(())
    }
}

pub fn flash(
    port: String,
    fw_path: String,
    progress: Option<&Sender<(usize, usize)>>,
) -> Result<(), Error> {
    let mut rx_buffer: Vec<u8> = vec![0; 256];
    let mut n_read: usize;

    // Try handshake three times
    for attempt in 0..4 {
        if attempt == 3 {
            return Err(Error::new(
                ErrorKind::Other,
                format!("[hello] Handshake failed after {:x} attempts!", attempt),
            ));
        }
        if hello(&port).is_ok() {
            break;
        }
    }

    // Open a new serial port instance
    let mut serial_port = serialport::new(port, 115_200)
        .timeout(Duration::from_millis(1000))
        .open()?;

    // Open firmware file
    let fw = std::fs::read(fw_path)?;

    // Send number of chunks
    let mut n_chunks: u8 = (fw.len() / PACKAGE_SIZE) as u8;
    if fw.len() % PACKAGE_SIZE > 0 {
        n_chunks += 1;
    }
    let mut tx_buffer = vec![0];
    tx_buffer[0] = n_chunks;
    let tx_buffer = fw_cmd(BootloaderCmd::SIZE as u8, 0, &tx_buffer, tx_buffer.len());
    _ = serial_port.write(&tx_buffer);

    n_read = 0;
    while n_read < 3 {
        n_read = n_read + serial_port.read(&mut rx_buffer[n_read..]).unwrap();
    }
    if rx_buffer[0] != 0xaa || rx_buffer[1] != BootloaderCmd::SIZE as u8 || rx_buffer[2] != 0x06 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("[nchunks] Bad response from bootloader: {:?}", rx_buffer),
        ));
    }

    // Send firmware in chunks
    for i in 0_usize..n_chunks as usize {
        let begin = i * PACKAGE_SIZE;
        let end = if fw.len() < (i + 1) * PACKAGE_SIZE {
            fw.len()
        } else {
            (i + 1) * PACKAGE_SIZE
        };
        // Original flasher advertises 1024B even in the last smaller chunk
        let tx_buffer = fw_cmd(
            BootloaderCmd::TRANSFER as u8,
            i as u8,
            &fw[begin..end],
            1024,
        );
        _ = serial_port.write(&tx_buffer);

        // Check response
        let mut rx_buffer: Vec<u8> = vec![0; 256];
        n_read = 0;
        while n_read < 3 {
            n_read = n_read + serial_port.read(&mut rx_buffer[n_read..]).unwrap();
        }
        if rx_buffer[0] != 0xaa
            || rx_buffer[1] != BootloaderCmd::TRANSFER as u8
            || rx_buffer[2] != 0x06
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("[chunks] Bad response from bootloader: {:?}", rx_buffer),
            ));
        }

        if progress.is_some() {
            match progress.unwrap().send((i + 1, n_chunks as usize)) {
                Err(e) => println!("Error when logging progress: {e}"),
                Ok(_) => (),
            }
        }
    }

    // Transfer complete
    let tx_buffer = fw_cmd(BootloaderCmd::REBOOT as u8, 0x00, &vec![], 0);
    _ = serial_port.write(&tx_buffer);

    // Check response
    let mut rx_buffer: Vec<u8> = vec![0; 256];
    n_read = 0;
    while n_read < 3 {
        n_read = n_read + serial_port.read(&mut rx_buffer[n_read..]).unwrap();
    }
    if rx_buffer[0] != 0xaa || rx_buffer[1] != BootloaderCmd::REBOOT as u8 || rx_buffer[2] != 0x06 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("[reboot] Bad response from bootloader: {:?}", rx_buffer),
        ));
    }

    Ok(())
}

fn xor_encrypt(data: &mut [u8], key: &[u8]) {
    for (i, byte) in data.iter_mut().enumerate() {
        let key_byte = key[i % 4];
        if *byte != 0 && *byte != 0xFF && *byte != key_byte && *byte != (key_byte ^ 0xFF) {
            *byte ^= key_byte;
        }
    }
}

pub fn wrap(input_path: &str, output_path: &str) -> Result<(), Error> {
    let mut input = File::open(input_path)?;
    let mut output = File::create(output_path)?;

    let file_size = input.seek(SeekFrom::End(0))?;
    input.seek(SeekFrom::Start(0))?;

    let package_count = (file_size + PACKAGE_SIZE as u64 - 1) / PACKAGE_SIZE as u64;
    let last_package_size = (file_size % PACKAGE_SIZE as u64) as usize;

    let mut buffer = vec![0u8; PACKAGE_SIZE];

    for i in 0..package_count {
        let current_package_size = if i == package_count - 1 && last_package_size > 0 {
            last_package_size
        } else {
            PACKAGE_SIZE
        };

        input.read_exact(&mut buffer[..current_package_size])?;

        if i >= 2 && i < package_count - 2 {
            if i % 3 == 1 {
                xor_encrypt(&mut buffer[..current_package_size], XOR_KEY1);
            } else if i % 3 == 2 {
                xor_encrypt(&mut buffer[..current_package_size], XOR_KEY2);
            }
        }

        output.write_all(&buffer[..current_package_size])?;
    }
    Ok(())
}

// pub fn write(port: &str, bin_path: &str) -> Result<(), Error> {
//     let mut serial_port = serialport::new(port, 115_200)
//         .timeout(Duration::from_millis(5000))
//         .open()?;
//
//     let handshake_str = "PROGRAM";
//     let model = "JC37"
//     _ = serial_port.write(&handshake_str);
//     _ = serial_port.write(&model);
//     _ = serial_port.write(&BootloaderCmd::U);
//
//     let mut rx_buffer: Vec<u8> = vec![0; 256];
//     _ = serial_port.read(&mut rx_buffer);
//     if rx_buffer[0] != 0x06 {
//         return Err(Error::new(
//             ErrorKind::Other,
//             format!("Bad response from radio: {:x}", rx_buffer[0]),
//         ));
//     }
//
//     // Begin download D
//     _ = serial_port.write(&BootloaderCmd::D);
//
//     sleep(0.5)
//
//     // TODO: reopen serial link
//
//     // Upload data to the firmware
//     let tx_buffer = data_cmd(
//         FirmwareCmd::HANDSHAKE as u8,
//         0,
//         handshake_str.as_bytes(),
//         handshake_str.as_bytes().len(),
//     );
//     _ = serial_port.write(&tx_buffer);
//
//     let mut rx_buffer: Vec<u8> = vec![0; 256];
//     _ = serial_port.read(&mut rx_buffer);
//     if rx_buffer[0] != 0xa5 {
//         return Err(Error::new(
//             ErrorKind::Other,
//             format!("Bad response from bootloader: {:x}", rx_buffer[0]),
//         ));
//     }
//
//     // Set address
//     let address_buf = vec![3];
//     address_buf[2] = address >> 16 & 0xff;
//     address_buf[1] = address >> 8 & 0xff;
//     address_buf[0] = address & 0xff;
//     let tx_buffer = data_cmd(
//         FirmwareCmd::SETADDRESS as u8,
//         0,
//         address_buf.as_bytes(),
//         address_buf.as_bytes().len(),
//     );
//     _ = serial_port.write(&tx_buffer);
//
//     _ = serial_port.read(&mut rx_buffer);
//     if rx_buffer[0] != 0xa5 {
//         return Err(Error::new(
//             ErrorKind::Other,
//             format!("Bad response from bootloader: {:x}", rx_buffer[0]),
//         ));
//     }
//
//     // TODO: Erase block
//
//     // TODO: Write data
//
//     // Send Over package
//     let over_str = "Over";
//     let tx_buffer = data_cmd(
//         FirmwareCmd::CMD_OVER as u8,
//         0,
//         over_str.as_bytes(),
//         over_str.as_bytes().len(),
//     );
//     _ = serial_port.write(&tx_buffer);
//
//     serial_port.close();
// }
