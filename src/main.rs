use std::fs;

fn main() -> std::io::Result<()> {
    
    let mut data: Vec<u8> = fs::read("data.srm")?;

    println!("Read {} bytes", data.len());

    let offset = 0x50;
    let value = data[offset];

    println!("Byte at 0x{:X} = {}", offset, value);    

    let start = 0x50;
    let size = 0x04;
    let slice = &data[start..(start + size)];

    for byte in slice {
        println!("0x{:X} ", byte)
    }
    println!();
    let write_byte = 0x50;
    let write_data = 0x69u8;
    data[write_byte] = write_data;

    let slice = &data[start..(start+size)];
    for (i, byte) in slice.iter().enumerate() {
        println!("0x{:04X}: 0x{:02X}", start + i,byte);
    }

    println!();


    let checksum_start = 0x2598;
    let checksum_end = 0x3522;
    let checksum_offset = 0x3523;
    let mut checksum: u8 = 0;

    for byte in &data[checksum_start..=checksum_end] {
        checksum = checksum.wrapping_add(*byte);
    }
    checksum = !checksum;

    let stored = data[checksum_offset];
    if checksum == stored {
        println!("Checksum OK");    
    }
    else {
        println!("Checksum mismatch: calculated 0x{:02X}, stored 0x{:02X}", checksum, stored);
    }

    println!("Checksum: {:02X}", checksum);


    Ok(())
}
