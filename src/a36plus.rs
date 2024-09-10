use serialport::SerialPort;
use std::env;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::io::{Error, ErrorKind};
use std::process;
use std::time::Duration;
use text_colorizer::*; // 0.17.1

const PACKAGE_SIZE: usize = 1024; // 1KB
const XOR_KEY1: &[u8; 4] = b"KDHT";
const XOR_KEY2: &[u8; 4] = b"RBGI";

enum BootloaderCmd {
    HELLO = 0x01,
    SIZE = 0x04,
    TRANSFER = 0x03,
    REBOOT = 0x45,
}

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
fn prep_cmd(cmd: u8, args: u8, input: &[u8], size: usize) -> Vec<u8> {
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

fn debug_print(buffer: &[u8], is_tx: bool) {
    if is_tx {
        print!("\nTX: ");
    } else {
        print!("\nRX: ");
    }
    let mut i = 0;
    for byte in buffer {
        print!("{:02x}, ", byte);
        i += 1;
        if (i % 16) == 0 {
            print!("\n");
        }
    }
}

pub fn flash(port: &str, fw_path: &str) -> Result<(), Error> {
    let mut serial_port = serialport::new(port, 115_200)
        .timeout(Duration::from_millis(5000))
        .open()?;

    let data = "BOOTLOADER";
    let tx_buffer = prep_cmd(
        BootloaderCmd::HELLO as u8,
        0,
        data.as_bytes(),
        data.as_bytes().len(),
    );
    _ = serial_port.write(&tx_buffer);

    let mut rx_buffer: Vec<u8> = vec![0; 256];
    _ = serial_port.read(&mut rx_buffer);
    if rx_buffer[0] != 0xaa {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Bad response from bootloader: {:x}", rx_buffer[0]),
        ));
    }

    if rx_buffer[0] != 0xaa || rx_buffer[1] != BootloaderCmd::HELLO as u8 || rx_buffer[2] != 0x06 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Bad response from bootloader: {:x}", rx_buffer[2]),
        ));
    }

    // Open firmware file
    let fw = std::fs::read(fw_path)?;

    // Send number of chunks
    let mut n_chunks: u8 = (fw.len() / PACKAGE_SIZE) as u8;
    if fw.len() % PACKAGE_SIZE > 0 {
        n_chunks += 1;
    }
    let mut tx_buffer = vec![0];
    tx_buffer[0] = n_chunks;
    let tx_buffer = prep_cmd(BootloaderCmd::SIZE as u8, 0, &tx_buffer, tx_buffer.len());
    _ = serial_port.write(&tx_buffer);

    let mut rx_buffer: Vec<u8> = vec![0; 256];
    _ = serial_port.read(&mut rx_buffer);
    if rx_buffer[0] != 0xaa || rx_buffer[1] != BootloaderCmd::SIZE as u8 || rx_buffer[2] != 0x06 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Bad response from bootloader: {:x}", rx_buffer[2]),
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
        let tx_buffer = prep_cmd(
            BootloaderCmd::TRANSFER as u8,
            i as u8,
            &fw[begin..end],
            1024,
        );
        _ = serial_port.write(&tx_buffer);

        // Check response
        let mut rx_buffer: Vec<u8> = vec![0; 256];
        _ = serial_port.read(&mut rx_buffer);
        if rx_buffer[0] != 0xaa
            || rx_buffer[1] != BootloaderCmd::TRANSFER as u8
            || rx_buffer[2] != 0x06
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Bad response from bootloader: {:x}", rx_buffer[2]),
            ));
        }
    }

    // Transfer complete
    let tx_buffer = prep_cmd(BootloaderCmd::REBOOT as u8, 0x00, &vec![], 0);
    _ = serial_port.write(&tx_buffer);

    // Check response
    let mut rx_buffer: Vec<u8> = vec![0; 256];
    _ = serial_port.read(&mut rx_buffer);
    if rx_buffer[0] != 0xaa || rx_buffer[1] != BootloaderCmd::REBOOT as u8 || rx_buffer[2] != 0x06 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Bad response from bootloader: {:x}", rx_buffer[2]),
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
