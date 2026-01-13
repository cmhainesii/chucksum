mod save_file;
mod textencoding;


use save_file::SaveFile;



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

    let player_name = save_file.read_string(0x2598, 0x50);
    println!("Player Name: {player_name}");

    save_file.write_string("Jerome", 0x2598, 0x50);
    let player_name = save_file.read_string(0x2598, 0x50);
    println!("Player Name: {player_name}");

    let rival_name = save_file.read_string(0x25F6, 0x50);
    println!("Rival Name: {rival_name}");
    let rival_name = "ASSHAT";
    
    save_file.write_string(rival_name, 0x25F6, 0x50);
    let rival_name = save_file.read_string(0x25F6, 0x50);
    println!("Rival Name: {rival_name}");


    save_file.save("altered.srm")?;


    Ok(())
}

