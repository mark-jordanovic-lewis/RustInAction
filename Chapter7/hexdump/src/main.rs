use std::fs::{File, OpenOptions};
use std::io::{prelude, Read};
use std::env;

const BYTES_PER_LINE: usize = 16;

fn main() -> std::io::Result<()> {
    let arg1 = env::args().nth(1);
    let fname = arg1.expect("usage: hexdump FILENAME");
    let mut f = File::open(&fname).expect("Unable to open file.");
    let mut buffer = [0; BYTES_PER_LINE];
    let mut position_in_input = 0;

    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", position_in_input);
        for byte in &buffer {
            match *byte {
                0x00 => print!(". "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte)
            }
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}

// Function for file opening options
// usage:
//   let mut file = rwa_options().open(FILENAME)?;
fn rwa_options() -> &mut OpenOptions {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
}