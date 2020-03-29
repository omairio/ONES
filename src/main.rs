use std::fs::File;
use std::io::{self, prelude::*, BufReader, Read};

fn main() -> io::Result<()>{
    
    // Init CPU
    // Init PPU

    let mut rom_path = String::new();
    
    let temp_rom = r"C:\Users\Omair\Downloads\Donkey_Kong\dk.nes";
    rom_path = String::from(temp_rom);
    println!("Attempting read of {}.", &rom_path);
    let file = File::open(&rom_path)?;
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::<u8>::new();
    let mut i = 0;
    while let Ok(num_bytes) = reader.read_until(0x0A as u8, &mut buffer){
        if (num_bytes == 0){
            break;
        }
        i = i + 1;
        for byte in &buffer {
            print!("{:#X?} ", byte);
        }
        buffer.clear();
        println!();
    }
    println!("Hello, world!");
    Ok(())
}
