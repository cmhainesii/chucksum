use std::fs;

use crate::items;
use crate::pokemon::{Pokemon, PokemonRaw};
use crate::textencoding;



pub struct SaveFile {
    data: Vec<u8>
}

#[derive(Debug)]
pub enum BagError {
    BagFull,
    InvalidQuantity(u8),
    InvalidItemId(u8),
}

#[derive(Debug)]
pub enum PartyError {
    LookupError
}

pub enum ItemStorage {
    PcBox,
    Bag,
}

pub struct ItemStorageOffsets {
    offset: usize,
    max_items: usize,
    count: u8,

}

impl std::fmt::Display for BagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BagError::BagFull => write!(f, "The bag is full"),
            BagError::InvalidQuantity(q) => write!(f, "Invalid Quantity: {}", q),
            BagError::InvalidItemId(id) => write!(f, "Invalid item ID: 0x{:02X}", id),
        }
    }
}

impl std::fmt::Display for PartyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartyError::LookupError => write!(f, "Party data is corrupted")
        }
    }
}

impl SaveFile {
    
    pub const GEN1_CHECKSUM_START: usize = 0x2598;
    pub const GEN1_CHECKSUM_END: usize  = 0x3522;
    pub const GEN1_CHECKSUM_OFFSET: usize = 0x3523;
    pub const GEN1_PLAERY_NAME_OFFSET: usize = 0x2598;
    pub const GEN1_RIVAL_NAME_OFFSET: usize = 0x25F6;
    pub const GEN1_MONEY_OFFSET: usize = 0x25F3;
    pub const GEN1_MONEY_MAX: u32 = 999_999;
    pub const GEN1_NAME_TERMINATOR: u8 = 0x50;
    
    
    // Item list constants - GEN 1
    pub const GEN1_BAG_OFFSET: usize = 0x25C9; // Beginning of Bag item list data.
    pub const GEN1_MAX_BAG_ITEMS: usize = 20;
    pub const GEN1_LIST_ITEM_SIZE: usize = 2;
    
    // This is the offset of the first item in the list relative to the list head
    pub const GEN1_ITEM_LIST_FIRST_ITEM: usize = 1;
    
    // Item box constants
    pub const GEN1_MAX_BOX_ITEMS: usize = 50;
    pub const GEN1_BOX_ITEMS_OFFSET: usize = 0x27E6;

    // Party related constants
    pub const GEN1_PARTY_DATA_OFFSET: usize = 0x2F2C; // Beginning of party data. Party count
    pub const _GEN1_MAX_PARTY_SIZE: usize = 6;
    pub const GEN1_PARTY_SPECIES_LIST_OFFSET: usize = 1; // Add this to party data offset to get first species in species list.
    pub const _GEN1_PARTY_LIST_TERMINATOR: u8 = 0xFF;
    pub const _GEN1_PARTY_START_TO_FIRST: usize = 8; // Add this to party data offset to get to first party pokemon

    
    
    pub fn new(filename: &str) -> std::io::Result<Self> {
        
        let data = fs::read(filename)?;
        Ok(Self { data })
    }
    
    pub fn len(&self) -> usize { self.data.len() }
    
    pub fn read_byte(&self, offset: usize) -> u8 {
        self.data[offset]
    }
    
    pub fn _read_bytes(&self, start: usize, end: usize) -> &[u8] {
        &self.data[start..=end]
    }
    
    pub fn write_byte(&mut self, offset: usize, value: u8) {
        self.data[offset] = value;
    }
    
    pub fn write_bytes(&mut self, offset: usize, data: &[u8] ) {
        let end = offset + data.len();
        
        if end > self.len() {
            return;
        }
        
        self.data[offset..end].copy_from_slice(data);
    }
    
    pub fn _as_slice(&self) -> &[u8] {
        &self.data
    }
    
    pub fn save(&mut self, filename: &str) -> std::io::Result<()> {
        
        // Calculate and update checksum. Important, do not skip or file will not be recognized as corrupted by the game
        self.write_byte(Self::GEN1_CHECKSUM_OFFSET, self.calculate_checksum(
            Self::GEN1_CHECKSUM_START, Self::GEN1_CHECKSUM_END));
            
            // Write save data to file
            fs::write(filename, &self.data)
        }
        
        fn calculate_checksum(&self, start: usize, end: usize) -> u8 {
            let mut checksum: u8 = 0;
            
            for byte in &self.data[start..=end] {
                checksum = checksum.wrapping_add(*byte);
            }
            println!("checksum: 0x{:04X}: 0x{:02X}", Self::GEN1_CHECKSUM_OFFSET, !checksum);
            !checksum
        }
        
        pub fn read_string(&self, start_offset: usize, terminator: u8) -> String {
            let mut current_offset = start_offset;
            let mut  current_char = self.read_byte(current_offset);
            let mut output = String::new();
            while current_offset < self.len() && current_char != terminator {
                output.push(textencoding::decode(current_char));
                current_offset += 1;
                current_char = self.read_byte(current_offset)
            }
            output
        }
        
        pub fn write_string(&mut self, input: &str, start_offset: usize, terminator: u8) {
            let mut current_offset = start_offset;
            
            for ch in input.chars() {
                if current_offset >= self.len() {
                    break;
                }
                let encoded = textencoding::encode(ch);
                self.write_byte(current_offset, encoded);
                current_offset += 1;
            }
            if current_offset < self.len() {
                self.write_byte(current_offset, terminator);
            }
        }
        
        pub fn bag_items_count(&self) -> u8 {
            self.read_byte(Self::GEN1_BAG_OFFSET).into()
            
        }
        
        pub fn box_items_count(&self) -> u8 {
            self.read_byte(Self::GEN1_BOX_ITEMS_OFFSET)
        }


        pub fn add_item(&mut self, dest: ItemStorage, item_id: u8, qty: u8) -> Result<(), BagError> {
            if qty == 0 {
                return Err(BagError::InvalidQuantity(qty))
            }

            let offsets = match dest {
                ItemStorage::Bag => {
                    ItemStorageOffsets {
                        offset: Self::GEN1_BAG_OFFSET,
                        max_items: Self::GEN1_MAX_BAG_ITEMS,
                        count: self.bag_items_count()
                    }
                },
                ItemStorage::PcBox => {
                    ItemStorageOffsets {
                        offset: Self::GEN1_BOX_ITEMS_OFFSET,
                        max_items: Self::GEN1_MAX_BOX_ITEMS,
                        count: self.box_items_count(),
                    }

                }
            };
            
            if offsets.count as usize >= Self::GEN1_MAX_BOX_ITEMS {
                return Err(BagError::BagFull);
            }
            
            // Check if we have a valid item id. If not display an error and abort.
            if !items::_is_valid_item(item_id) {
                return Err(BagError::InvalidItemId(item_id));
            }
            let next_free_slot = (offsets.offset + Self::GEN1_ITEM_LIST_FIRST_ITEM)+ (Self::GEN1_LIST_ITEM_SIZE * offsets.count as usize);
            let item_data = [item_id, qty];
            
            self.write_bytes(next_free_slot, &item_data);
            self.write_byte(offsets.offset, offsets.count + 1);
            
            Ok(())
            
        }

        pub fn list_items(&self, destination: ItemStorage) -> String {
            let mut output = String::new();
            let mut current_slot = 0;
            
            
            // set offsets for correct destination (box/bag)
            let dest_offsets = match destination {
                ItemStorage::Bag => {
                    ItemStorageOffsets {
                        max_items: Self::GEN1_MAX_BAG_ITEMS,
                        offset : Self::GEN1_BAG_OFFSET,
                        count: self.bag_items_count()
                    }
                }
                ItemStorage::PcBox => {
                    ItemStorageOffsets {
                        max_items: Self::GEN1_MAX_BOX_ITEMS,
                        offset: Self::GEN1_BOX_ITEMS_OFFSET,
                        count: self.box_items_count()
                    }
                }
            };

            
            if dest_offsets.count > 0 {
                let mut current_offset = dest_offsets.offset + Self::GEN1_ITEM_LIST_FIRST_ITEM;
                let last_slot_offset = current_offset + (Self::GEN1_LIST_ITEM_SIZE * dest_offsets.max_items);
                
                while current_offset <= last_slot_offset && current_slot < dest_offsets.count  {
                    let current_item = items::get_item_name(self.read_byte(current_offset));
                    let item_qty = self.read_byte(current_offset + 1);
                    
                    output.push_str(format!("{current_item} - Qty: {item_qty}\n").as_str());
                    
                    current_offset += 2;
                    current_slot += 1;
                }
                
            }
            output
        }

        pub fn set_player_name(&mut self, input: &str) {
            self.write_string(input, Self::GEN1_PLAERY_NAME_OFFSET, 0x50);
        }

        pub fn get_player_name(&self) -> String {
            self.read_string(Self::GEN1_PLAERY_NAME_OFFSET, 0x50)
        }

        pub fn set_rival_name(&mut self, input: &str) {
            self.write_string(input, Self::GEN1_RIVAL_NAME_OFFSET, Self::GEN1_NAME_TERMINATOR);
        }

        pub fn get_rival_name(&self) -> String {
            self.read_string(Self::GEN1_RIVAL_NAME_OFFSET, Self::GEN1_NAME_TERMINATOR)
        }

        fn _bcd_byte_to_decimal(byte: u8) -> u8 {
            let high = (byte >> 4) & 0x0F;
            let low = byte & 0x0F;
            high * 10 + low
        }

        fn _decimal_pair_to_bcd(value: u8) -> u8 {
            let tens = value / 10;
            let ones = value % 10;

            (tens << 4) | ones
        }



        pub fn get_money(&self) -> u32 {
            let offset = Self::GEN1_MONEY_OFFSET;

            let b1 = self.data[offset];
            let b2 = self.data[offset + 1];
            let b3 = self.data[offset + 2];

            let d1 = Self::_bcd_byte_to_decimal(b1) as u32;
            let d2 = Self::_bcd_byte_to_decimal(b2) as u32;
            let d3 = Self::_bcd_byte_to_decimal(b3) as u32;

            d1 * 10_000 + d2 * 100 + d3
        }

        fn _money_to_bcd_bytes(mut money: u32) -> [u8; 3] {
            // Cap to Gen 1 max
            money = money.min(Self::GEN1_MONEY_MAX);

            let hundred_thousands = (money / 100_000) as u8;
            let ten_thousands = ((money / 10_000) % 10) as u8;
            let thousands = ((money / 1000) % 10) as u8;
            let hundreds = ((money / 100) % 10) as u8;
            let tens = ((money / 10) % 10) as u8;
            let ones = (money % 10) as u8;

            [
                (hundred_thousands << 4) | ten_thousands,
                (thousands << 4) | hundreds,
                (tens << 4) | ones,
            ]
        }

        pub fn set_money(&mut self, money: u32) {
            let bytes = Self::_money_to_bcd_bytes(money);
            self.write_bytes(Self::GEN1_MONEY_OFFSET, &bytes);
        }

        pub fn get_party_species_names(&self) -> Result<Vec<&'static str>, PartyError> {

            let count = self.read_byte(Self::GEN1_PARTY_DATA_OFFSET);
            if count <= 0 || count > 6 {
                return Err(PartyError::LookupError);
            }
            let mut species_names: Vec<&'static str> = Vec::new();
            let count = self.read_byte(Self::GEN1_PARTY_DATA_OFFSET);
            let current_offset = Self::GEN1_PARTY_DATA_OFFSET + Self::GEN1_PARTY_SPECIES_LIST_OFFSET;
            for i in 0..count as usize {
                let species_id = self.read_byte(current_offset + i);
                species_names.push(Pokemon::get_species_name(species_id));
            }
            if species_names.len() == 0 || species_names.len() > 6 {
                return Err(PartyError::LookupError);
            }
            Ok(species_names)
        }

        pub fn get_party_count(&self) -> usize {
            self.read_byte(Self::GEN1_PARTY_DATA_OFFSET) as usize
        }

        pub fn get_party_pokemon_data(&self) -> Result<Vec<Pokemon>, PartyError> {
            let count = self.get_party_count();
            
            if count <= 0 || count > 6 {
                return Err(PartyError::LookupError);
            }

            let mut offset = Pokemon::FIRST_PKMN_OFFSET;
            let mut list = Vec::new();

            for _ in 0..count {
                let raw = self.read_pokemon_raw(offset);
                let pokemon = Pokemon::from_raw(raw);
                list.push(pokemon);
                offset += Pokemon::NEXT_PARTY_PKMN;
            }

            Ok(list)
        }

        // pub fn get_party_pokemon_data(&self) -> Result<Vec<Pokemon>, PartyError> {
        //     let mut current_pkmn = Pokemon::FIRST_PKMN_OFFSET;
        //     let mut pokemon_list = Vec::new();
        //     let count = self.get_party_count();
        //     if count > 6 || count <= 0 {
        //         return Err(PartyError::LookupError);
        //     }           
            
        //     for _ in 0..count {

                
        //         let pokemon = Pokemon {
        //             species_id: self.read_byte(current_pkmn + Pokemon::SPECIES_ID),
        //             current_hp: self.read_u16_be(current_pkmn + Pokemon::CURRENT_HP),
        //             level: self.read_byte(current_pkmn + Pokemon::LEVEL),
        //             status: self.read_byte(current_pkmn + Pokemon::STATUS),
        //             pkmn_type_1: self.read_byte(current_pkmn + Pokemon::TYPE_1),
        //             pkmn_type_2: self.read_byte(current_pkmn + Pokemon::TYPE_2),
        //             catch_rate: self.read_byte(current_pkmn + Pokemon::CATCH_RATE),
        //             move_index1: self.read_byte(current_pkmn + Pokemon::MOVE_INDEX_1),
        //             move_index2: self.read_byte(current_pkmn + Pokemon::MOVE_INDEX_2),
        //             move_index3: self.read_byte(current_pkmn + Pokemon::MOVE_INDEX_3),
        //             move_index4: self.read_byte(current_pkmn + Pokemon::MOVE_INDEX_4),
        //             ot_id: self.read_u16_be(current_pkmn + Pokemon::OT_ID),
        //             experience_pts: self.read_u24_be(current_pkmn + Pokemon::EXPERIENCE_PTS),
        //             hp_stat_exp: self.read_u16_le(current_pkmn + Pokemon::HP_STAT_EXP),
        //             attack_stat_exp: self.read_u16_le(current_pkmn + Pokemon::ATTACK_STAT_EXP),
        //             defense_stat_exp: self.read_u16_le(current_pkmn + Pokemon::DEFENSE_STAT_EXP),
        //             speed_stat_exp: self.read_u16_le(current_pkmn + Pokemon::SPEED_STAT_EXP),
        //             special_stat_exp: self.read_u16_le(current_pkmn + Pokemon::SPECIAL_STAT_EXP),
        //             attack_iv: self.read_u8_high(current_pkmn + Pokemon::IV_1),
        //             defense_iv: self.read_u8_low(current_pkmn + Pokemon::IV_1),
        //             speed_iv: self.read_u8_high(current_pkmn + Pokemon::IV_2),
        //             special_iv: self.read_u8_low(current_pkmn + Pokemon::IV_2),
        //         };

        //         current_pkmn += 0x2C;
        //         pokemon_list.push(pokemon);
        //     }
        //     Ok(pokemon_list)


        // }

        // fn read_u16_be(&self, offset: usize) -> u16 {
        //     let b = self.read_bytes(offset, offset + 1);
        //     u16::from_be_bytes([b[0], b[1]])
        // }

        // fn read_u16_le(&self, offset: usize) -> u16 {
        //     let b = self.read_bytes(offset, offset + 1);
        //     u16::from_le_bytes([b[0], b[1]])
        // }

        // fn read_u24_be(&self, offset: usize) -> u32 {
        //     let b = self.read_bytes(offset, offset + 2);
        //     ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32
        // }

        // fn read_u8_high(&self, offset: usize) -> u8 {
        //     let b = self.read_byte(offset);
        //     (b >> 4) & 0x0F
        // }

        // fn read_u8_low(&self, offset: usize) -> u8 {
        //     let b = self.read_byte(offset);
        //     b & 0x0F
        // }

        pub fn read_pokemon_raw(&self, offset: usize) -> PokemonRaw {
            let mut data = [0u8; 0x2C];
            data.copy_from_slice(&self.data[offset..offset + 0x2C]);
            PokemonRaw::new(data)
        }


        
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod decimal_pair_to_bcd {
            use super::*;
            #[test]
            fn converts_single_digit() {
                assert_eq!(SaveFile::_decimal_pair_to_bcd(0), 0x00);
                assert_eq!(SaveFile::_decimal_pair_to_bcd(5), 0x05);
                assert_eq!(SaveFile::_decimal_pair_to_bcd(9), 0x09);
            }

            #[test]
            fn converts_two_digits() {
                assert_eq!(SaveFile::_decimal_pair_to_bcd(12), 0x12);
                assert_eq!(SaveFile::_decimal_pair_to_bcd(34), 0x34);
                assert_eq!(SaveFile::_decimal_pair_to_bcd(99), 0x99);
            }

            #[test]
            fn handles_round_numbers() {
                assert_eq!(SaveFile::_decimal_pair_to_bcd(10), 0x10);
                assert_eq!(SaveFile::_decimal_pair_to_bcd(20), 0x20);
                assert_eq!(SaveFile::_decimal_pair_to_bcd(90), 0x90);
            }

        mod bcd_byte_to_decimal {
            use super::*;
            // _bcd_byte_to_decimal tests:
            #[test]
            fn test_bcd_byte_to_decimal() {
                assert_eq!(SaveFile::_bcd_byte_to_decimal(0x00), 0);
                assert_eq!(SaveFile::_bcd_byte_to_decimal(0x05), 5);
                assert_eq!(SaveFile::_bcd_byte_to_decimal(0x10), 10);
                assert_eq!(SaveFile::_bcd_byte_to_decimal(0x20), 20);
                assert_eq!(SaveFile::_bcd_byte_to_decimal(0x55), 55);
                assert_eq!(SaveFile::_bcd_byte_to_decimal(0x99), 99);
            }
        }
    }
}