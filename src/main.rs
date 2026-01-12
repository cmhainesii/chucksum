mod savefile;



use savefile::SaveFile;



fn main() -> std::io::Result<()> {
    
    let mut save_file = SaveFile::new("data.srm")?;

    println!("Read {} bytes", save_file.len());

    let offset = 0x3000;
    let value = save_file.read_byte(offset);
    println!("0x{:04X}: 0x{:02X}", offset, value);

    save_file.write_byte(offset, 66);
    

    let value = save_file.read_byte(offset);
    println!("Afte write byte:");
    println!("0x{:04X}: 0x{:02X}", offset, value);
    println!();

    

    save_file.save("altered.srm")?;
    // let checksum_value = calculate_checksum(&data, checksum_start, checksum_end);
    // let checksum_validated = validate_checksum(&data, checksum_value, checksum_offset);


    Ok(())
}

