mod savefile;
mod checksum;


use savefile::SaveFile;

use crate::checksum::validate_checksum;

fn main() -> std::io::Result<()> {
    
    let mut save_file = SaveFile::new("data.srm")?;

    println!("Read {} bytes", save_file.len());

    let offset = 0x50;
    let value = save_file.read_byte(offset);
    println!("0x{:04X}: 0x{:02X}", offset, value);

    save_file.write_byte(offset, 66);

    let value = save_file.read_byte(offset);
    println!("Afte write byte:");
    println!("0x{:04X}: {:02X}", offset, value);
    println!();


    let checksum_start = 0x2598;
    let checksum_end = 0x3522;
    let checksum_offset = 0x3523;

    let stored_checksum = save_file.read_byte(checksum_offset);
    let calculated_checksum = checksum::calculate_checksum(save_file.as_slice(), checksum_start, checksum_end);
    let checksum_valid = validate_checksum(&save_file.as_slice(), calculated_checksum, checksum_offset);
    if checksum_valid {
        println!("Checksum ok: 0x{:04X}: {:02X}", checksum_offset, calculated_checksum);
    }
    else {
        println!("Checksum invalid. Stored: {stored_checksum} Calculated: {calculated_checksum}");
    }

    save_file.save("altered.srm")?;
    // let checksum_value = calculate_checksum(&data, checksum_start, checksum_end);
    // let checksum_validated = validate_checksum(&data, checksum_value, checksum_offset);


    Ok(())
}

