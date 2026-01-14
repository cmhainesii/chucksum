mod save_file;
mod textencoding;
mod items;


use save_file::SaveFile;



fn main() -> std::io::Result<()> {
    
    let mut save_file = SaveFile::new("data.srm")?;
    
    println!("Read {} bytes", save_file.len());
    
    let offset = 0x3000;
    let value = save_file.read_byte(offset);
    println!("0x{:04X}: 0x{:02X}", offset, value);
    
    //save_file.write_byte(offset, 66);
    
    
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
    
    
    
    
    println!{"# of bag items: {}", save_file.bag_items_count()};
    
    // Add 10 Pokeballs to first empty bag slot:
    match save_file.add_item_to_bag(0x01, 10) {
        Ok(_) => println!("Added item to bag successfully."),
        Err(e) => println!("Failed to add item: {e}"),
    }

    match save_file.add_item_to_bag(0x53, 96) {
        Ok(_) => println!("Added second item to bag successfully"),
        Err(e) => println!("Failed ot add item: {e}"),
    }
    
    println!{"# of bag items: {}", save_file.bag_items_count()};
    
    println!("Try listing bag items:\n\n");
    println!("{}", save_file.list_bag_items());
    
    // Save to file 'pokemon red.sav'. Will automatically update main checksum.
    save_file.save("pokemon red.sav")?;
    
    
    Ok(())
}