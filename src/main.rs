mod save_file;
mod textencoding;
mod items;
mod pokemon;
mod offsets;

use num_format::Locale;
use num_format::ToFormattedString;
use save_file::SaveFile;
use pokemon::Pokemon;


use crate::{pokemon::StatusCondtion, save_file::ItemStorage};



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
    
    //save_file.write_string("Jerome", 0x2598, 0x50);
    save_file.set_player_name("Jerome");
    let player_name = save_file.get_player_name();
    println!("Player Name: {player_name}");
    
    let rival_name = save_file.get_rival_name();
    println!("Rival Name: {rival_name}");
    save_file.set_rival_name("ASSHAT");
    
    
    let rival_name = save_file.get_rival_name();
    println!("Rival Name: {rival_name}");
    
    
    
    println!{"# of bag items: {}", save_file.bag_items_count()};
    
    // Add 10 Pokeballs to first empty bag slot:
    match save_file.add_item(ItemStorage::Bag, 0x01, 10) {
        Ok(_) => println!("Added item to bag successfully."),
        Err(e) => println!("Failed to add item: {e}"),
    }

    match save_file.add_item(ItemStorage::Bag, 0x53, 96) {
        Ok(_) => println!("Added second item to bag successfully"),
        Err(e) => println!("Failed to add item: {e}"),
    }
    
    println!{"# of bag items: {}", save_file.bag_items_count()};
    
    println!("Try listing bag items:\n\n");
    println!("{}", save_file.list_items(ItemStorage::Bag));

    println!("Try printing box items: ");
    println!("{}", save_file.list_items(ItemStorage::PcBox));

    match save_file.add_item(ItemStorage::PcBox, 0x01, 98) {
        Ok(_) => println!("Added item to box successfully."),
        Err(e) => println!("Failed to add item: {e}")
    }
    
    println!("{}", save_file.list_items(ItemStorage::PcBox));



    
    println!("Money: ${}", save_file.get_money().to_formatted_string(&Locale::en));

    save_file.set_money(986_186);

    // Print out party species names.
    match save_file.get_party_species_names() {
        Ok(names) => {
            for name in names {
                println!("{name}");
            }
        }
        Err(e) => println!("Lookup error: {e}")
    }
    
    println!();
    println!("-------------------");
    println!("Party Pokémon Data");
    println!("-------------------");
    println!();

    match save_file.get_party_pokemon_data() {
        Ok(pokemon_list) => {
            for pokemon in pokemon_list {
                println!("          Species: {}", Pokemon::get_species_name(pokemon.species_id));
                println!("       Current HP: {}", pokemon.current_hp);
                println!("           Max HP: {}", pokemon.max_hp);
                println!("            Level: {}", pokemon.level);
                println!("           Status: {}", StatusCondtion::from_byte(pokemon.status));
                println!("             Type: {}", Pokemon::get_type_name(pokemon.pkmn_type_1));
                println!("            Type2: {}", Pokemon::get_type_name(pokemon.pkmn_type_2));
                println!("       Catch Rate: {}", pokemon.catch_rate);
                println!("           Move 1: {}", Pokemon::get_move_name(pokemon.move_index1));
                println!("           Move 2: {}", Pokemon::get_move_name(pokemon.move_index2));
                println!("           Move 3: {}", Pokemon::get_move_name(pokemon.move_index3));
                println!("           Move 4: {}", Pokemon::get_move_name(pokemon.move_index4));
                println!("            OT ID: {}", pokemon.ot_id); 
                println!("Experience Points: {}", pokemon.experience_pts.to_formatted_string(&Locale::en));
                println!("      HP Stat Exp: {}", pokemon.hp_stat_exp.to_formatted_string(&Locale::en));
                println!("  Attack Stat Exp: {}", pokemon.attack_stat_exp.to_formatted_string(&Locale::en));
                println!(" Defense Stat Exp: {}", pokemon.defense_stat_exp.to_formatted_string(&Locale::en));
                println!("   Speed Stat Exp: {}", pokemon.speed_stat_exp.to_formatted_string(&Locale::en));
                println!(" Special Stat Exp: {}", pokemon.special_stat_exp.to_formatted_string(&Locale::en));
                println!("        Attack IV: {}", pokemon.attack_iv);
                println!("       Defense IV: {}", pokemon.defense_iv);
                println!("         Speed IV: {}", pokemon.speed_iv);
                println!("       Special IV: {}\n", pokemon.special_iv)
                
            }
        },
        Err(e) => println!("Lookup error: {e}")
    }

    match save_file.badges_strings() {
        Ok(badges) => {
            for badge in badges {
                println!("{badge}");
            }
        },
        Err(e) => println!("Error: {e}")
    }

    println!("Player ID: {}", save_file.get_player_id());
    // Save to file 'pokemon red.sav'. Will automatically update main checksum.
    save_file.save("pokemon red.sav")?;

    
    
    
    Ok(())
}