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
    let checksum_value = calculate_checksum(&data, checksum_start, checksum_end);
    let checksum_validated = validate_checksum(&data, checksum_value, checksum_offset);

    let stored = data[checksum_offset];
    if checksum_validated {
        println!("Checksum OK");    
    }
    else {
        println!("Checksum mismatch: calculated 0x{:02X}, stored 0x{:02X}", checksum_value, stored);
    }

    println!("Checksum: {:02X}", checksum_value);

    data[checksum_offset] = 0x69;

    fs::write("data_modified.srm", &data)?;

    println!("Saved modified file as data_modified.srm");


    Ok(())
}

fn calculate_checksum(data: &[u8], start: usize, end: usize) -> u8 {
    let mut checksum: u8 = 0;

    for byte in &data[start..=end] {
        checksum = checksum.wrapping_add(*byte);
    }
    !checksum
}

fn validate_checksum(data: &[u8], calculated: u8, checksum_offset: usize) -> bool {
    let stored_checksum = data[checksum_offset];
    stored_checksum == calculated    
}
