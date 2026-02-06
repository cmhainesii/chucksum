use std::fs;

use bitflags::bitflags;

use crate::items;
use crate::pokemon::Pokemon;
use crate::pokemon::PokemonRaw;
use crate::textencoding;
use crate::offsets;



pub struct SaveFile {
    data: Vec<u8>
}



bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Badges: u8 {
        const BOULDER = 0b0000_0001;
        const CASCADE = 0b0000_0010;
        const THUNDER = 0b0000_0100;
        const RAINBOW = 0b0000_1000;
        const SOUL = 0b0001_0000;
        const MARSH = 0b0010_0000;
        const VOLCANO = 0b0100_0000;
        const EARTH = 0b1000_0000;
    }
}

#[derive(Debug)]
pub enum BagError {
    BagFull,
    InvalidQuantity(u8),
    InvalidItemId(u8),
}

#[derive(Debug)]
pub enum PokemonError {
    LookupError,
    InvalidBoxNumber,
    InvalidData,
    InvalidPartySlot,
    PokemonBoxFull,
    BoxInUse,
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

impl std::fmt::Display for PokemonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PokemonError::LookupError => write!(f, "Party data is corrupted"),
            PokemonError::InvalidBoxNumber => write!(f, "Invalid box number. Should be an integer between 1 and 12"),
            PokemonError::InvalidData => write!(f, "Invalid or corrupted data"),
            PokemonError::InvalidPartySlot => write!(f, "Invalid party slot! Should be an integer between 1 and 6"),
            PokemonError::PokemonBoxFull => write!(f, "Pokemon box is full! Aborting."),
            PokemonError::BoxInUse => write!(f, "Cannot copy pokemon to the current box. Select another box with a free slot and try again."),
        }
    }
}

impl SaveFile {
    
    pub fn new(filename: &str) -> std::io::Result<Self> {
        
        let data = fs::read(filename)?;
        Ok(Self { data })
    }
    
    pub fn len(&self) -> usize { self.data.len() }
    
    pub fn read_byte(&self, offset: usize) -> u8 {
        self.data[offset]
    }
    
    pub fn read_u16_be(&self, offset: usize) -> u16 {
        u16::from_be_bytes([self.data[offset], self.data[offset + 1]])
    }
    
    pub fn _read_u16_le(&self, offset: usize) -> u16 {
        u16::from_le_bytes([self.data[offset], self.data[offset + 1]])
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
        self.write_byte(offsets::CHECKSUM_OFFSET, self.calculate_checksum(offsets::CHECKSUM_START, offsets::CHECKSUM_END));

        // println!("All boxes calculated: 0x{:02X}",self.calculate_checksum(0x4000, 0x5A4B));
        // println!("Should be: 0x{:02X}", self.read_byte(0x5A4C));

        // println!("Box 1 calculated: 0x{:02X}",self.calculate_checksum(0x4000, 0x4461));
        // println!("Should be: 0x{:02X}", self.read_byte(0x5A4D));

        self.update_box_checksums();


        
        // Write save data to file
        fs::write(filename, &self.data)
    }
    
    fn calculate_checksum(&self, start: usize, end: usize) -> u8 {
        let mut checksum: u8 = 0;
        
        for byte in &self.data[start..=end] {
            checksum = checksum.wrapping_add(*byte);
        }
        // println!("checksum: 0x{:04X}: 0x{:02X}", offsets::CHECKSUM_OFFSET, !checksum);
        !checksum
    }

    fn update_box_checksums(&mut self) {
        // Calculate and update whole bank checksums for bank 2
        let mut checksum = self.calculate_checksum(0x4000, 0x5A4B);
        self.write_byte(offsets::BANK2_WHOLE_CHECKSUM, checksum);

        checksum = self.calculate_checksum(0x6000, 0x7A4B);
        self.write_byte(offsets::BANK3_WHOLE_CHECKSUM, checksum);
        
        let mut current_offset = 0x4000;
        let current_to_end = 0x461;
        let mut current_write_offset = offsets::BANK2_WHOLE_CHECKSUM + 1;

        for _ in 0..6 {
            checksum = self.calculate_checksum(current_offset, current_offset + current_to_end);
            self.write_byte(current_write_offset, checksum);
            current_offset += offsets::BOX_NEXT_BOX;
            current_write_offset += 1;
        }

        current_offset = 0x6000;
        current_write_offset = offsets::BANK3_WHOLE_CHECKSUM + 1;

        for _ in 0..6 {
            checksum = self.calculate_checksum(current_offset, current_offset + current_to_end);
            self.write_byte(current_write_offset, checksum);
            current_offset += offsets::BOX_NEXT_BOX;
            current_write_offset += 1;
        }

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
        self.read_byte(offsets::BAG_OFFSET).into()
        
    }
    
    pub fn box_items_count(&self) -> u8 {
        self.read_byte(offsets::BOX_ITEMS_OFFSET)
    }
    
    
    pub fn add_item(&mut self, dest: ItemStorage, item_id: u8, qty: u8) -> Result<(), BagError> {
        if qty == 0 {
            return Err(BagError::InvalidQuantity(qty))
        }
        
        let offsets = match dest {
            ItemStorage::Bag => {
                ItemStorageOffsets {
                    offset: offsets::BAG_OFFSET,
                    max_items: offsets::MAX_BAG_ITEMS,
                    count: self.bag_items_count()
                }
            },
            ItemStorage::PcBox => {
                ItemStorageOffsets {
                    offset: offsets::BOX_ITEMS_OFFSET,
                    max_items: offsets::MAX_BOX_ITEMS,
                    count: self.box_items_count(),
                }
                
            }
        };
        
        if offsets.count as usize >= offsets::MAX_BOX_ITEMS {
            return Err(BagError::BagFull);
        }
        
        // Check if we have a valid item id. If not display an error and abort.
        if !items::_is_valid_item(item_id) {
            return Err(BagError::InvalidItemId(item_id));
        }
        let next_free_slot = (offsets.offset + offsets::ITEM_LIST_FIRST_ITEM)+ (offsets::LIST_ITEM_SIZE * offsets.count as usize);
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
                    max_items: offsets::MAX_BAG_ITEMS,
                    offset : offsets::BAG_OFFSET,
                    count: self.bag_items_count()
                }
            }
            ItemStorage::PcBox => {
                ItemStorageOffsets {
                    max_items: offsets::MAX_BOX_ITEMS,
                    offset: offsets::BOX_ITEMS_OFFSET,
                    count: self.box_items_count()
                }
            }
        };
        
        
        if dest_offsets.count > 0 {
            let mut current_offset = dest_offsets.offset + offsets::ITEM_LIST_FIRST_ITEM;
            let last_slot_offset = current_offset + (offsets::LIST_ITEM_SIZE * dest_offsets.max_items);
            
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
        self.write_string(input, offsets::PLAYER_NAME_OFFSET, 0x50);
    }
    
    pub fn get_player_name(&self) -> String {
        self.read_string(offsets::PLAYER_NAME_OFFSET, 0x50)
    }
    
    pub fn set_rival_name(&mut self, input: &str) {
        self.write_string(input, offsets::RIVAL_NAME_OFFSET, offsets::NAME_TERMINATOR);
    }
    
    pub fn get_rival_name(&self) -> String {
        self.read_string(offsets::RIVAL_NAME_OFFSET, offsets::NAME_TERMINATOR)
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
        let offset = offsets::MONEY_OFFSET;
        
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
        money = money.min(offsets::MONEY_MAX);
        
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
        self.write_bytes(offsets::MONEY_OFFSET, &bytes);
    }
    
    pub fn get_party_species_names(&self) -> Result<Vec<&'static str>, PokemonError> {
        
        let count = self.read_byte(offsets::PARTY_DATA_OFFSET);
        if count <= 0 || count > 6 {
            return Err(PokemonError::InvalidData);
        }
        let mut species_names: Vec<&'static str> = Vec::new();
        let count = self.read_byte(offsets::PARTY_DATA_OFFSET);
        let current_offset = offsets::PARTY_DATA_OFFSET + offsets::PARTY_SPECIES_LIST_OFFSET;
        for i in 0..count as usize {
            let species_id = self.read_byte(current_offset + i);
            species_names.push(Pokemon::get_species_name(species_id));
        }
        if species_names.len() == 0 || species_names.len() > 6 {
            return Err(PokemonError::InvalidData);
        }
        Ok(species_names)
    }
    
    pub fn get_party_count(&self) -> usize {
        self.read_byte(offsets::PARTY_DATA_OFFSET) as usize
    }
    
    pub fn get_current_box_pokemon_count(&self) -> usize {
        self.read_byte(offsets::BOX_CURRENT_DATA_OFFSET) as usize
    }
    
    pub fn get_box_pokemon_count(&self, box_number: usize) -> usize {

        if box_number == self.get_current_box() {
            let offset = offsets::PARTY_DATA_OFFSET;
            return self.read_byte(offset) as usize;
        }
        
        if box_number > 0 && box_number <= 6 {
            let offset = offsets::BOX_1_DATA_OFFSET + (offsets::BOX_NEXT_BOX * (box_number - 1));
            return self.read_byte(offset) as usize;
        }
        else if box_number > 6 && box_number <= 12 {
            let offset = offsets::BOX_7_DATA_OFFSET + (offsets::BOX_NEXT_BOX * (box_number - 7));
            return self.read_byte(offset) as usize;
        }
        return 0;
    }
    
    pub fn get_box_pokemon_count_offset(&self, box_number: usize) -> usize {
        if box_number == self.get_current_box() {
            return offsets::PARTY_DATA_OFFSET;
        }
        else if box_number <= 6 {
            return offsets::BOX_1_DATA_OFFSET + (offsets::BOX_NEXT_BOX * (box_number - 1))
        }
        else {
            return offsets::BOX_7_DATA_OFFSET + (offsets::BOX_NEXT_BOX * (box_number - 7));
        }
    }
    
    pub fn get_party_pokemon_data(&self) -> Result<Vec<Pokemon>, PokemonError> {
        let count = self.get_party_count();
        
        if count <= 0 || count > offsets::MAX_PARTY_SIZE {
            return Err(PokemonError::InvalidData);
        }
        
        let mut offset = offsets::PARTY_FIRST_PKMN;
        let mut list = Vec::new();
        
        for _ in 0..count {
            let raw = self.read_party_pokemon_raw(offset);
            let pokemon = Pokemon::from_raw(raw);
            list.push(pokemon);
            offset += offsets::PARTY_NEXT_PKMN;
        }
        
        Ok(list)
    }
    
    
    pub fn get_current_box_pokemon_data(&self) -> Result<Vec<Pokemon>, PokemonError> {
        let count = self.get_current_box_pokemon_count();
        
        if count <= 0 || count > offsets::MAX_POKEMON_BOX_SIZE {
            return Err(PokemonError::LookupError);
        }
        
        let mut offset = offsets::BOX_CURRENT_FIRST_PKMN;
        let mut list = Vec::new();
        
        for _ in 0..count {
            let raw = self.read_box_pokemon_raw(offset);
            let pokemon = Pokemon::from_raw(raw);
            list.push(pokemon);
            offset += offsets::BOX_NEXT_PKMN;
        }
        Ok(list)
    }
    
    pub fn get_box_pokemon_data(&self, box_number: usize) -> Result<Vec<Pokemon>, PokemonError> {
        
        if box_number < 1 || box_number > 12 {
            return Err(PokemonError::InvalidBoxNumber);
        }
        let count = self.get_box_pokemon_count(box_number);
        let mut list = Vec::new();
        
        // Set offset to the first byte in the box structure
        // Handle box 1-6:
        let mut offset: usize;
        if box_number <= 6 {
            offset = offsets::BOX_1_DATA_OFFSET + (offsets::BOX_NEXT_BOX * (box_number - 1));
            
        }
        // Handle box 7-12
        else {
            offset = offsets::BOX_7_DATA_OFFSET + (offsets::BOX_NEXT_BOX * (box_number - 7));
        }
        
        // Skip to begining of first pokemon's data
        offset += offsets::BOX_START_TO_FIRST; 
        
        
        for _ in 0..count {
            let raw = self.read_box_pokemon_raw(offset);
            let pokemon = Pokemon::from_raw(raw);
            list.push(pokemon);
            offset += offsets::BOX_NEXT_PKMN;
        }
        
        Ok(list)
    }
    
    pub fn read_party_pokemon_raw(&self, offset: usize) -> PokemonRaw {
        let mut data = [0u8; offsets::PARTY_NEXT_PKMN];
        data.copy_from_slice(&self.data[offset..offset + offsets::PARTY_NEXT_PKMN]);
        PokemonRaw::new(data)
    }
    
    pub fn read_box_pokemon_raw(&self, offset: usize) -> PokemonRaw {
        let mut data = [0u8; offsets::PARTY_NEXT_PKMN];
        data[..offsets::BOX_NEXT_PKMN].copy_from_slice(&self.data[offset..offset + offsets::BOX_NEXT_PKMN]);
        data[0x21] = self.data[offset + 0x03];
        
        
        PokemonRaw::new(data)
        
    }
    
    pub fn get_badges(&self) -> Badges {
        Badges::from_bits_truncate(self.read_byte(offsets::BADGES))
    }
    
    pub fn badges_strings(&self) -> Result<Vec<&'static str>, PokemonError> {
        let b = self.get_badges();
        // let b = Badges::from_bits_truncate(0b0010_1111);
        let names = [
        (Badges::BOULDER, "Boulder"),
        (Badges::CASCADE, "Cascade"),
        (Badges::THUNDER, "Thunder"),
        (Badges::RAINBOW, "Rainbow"),
        (Badges::SOUL, "Soul"),
        (Badges::MARSH, "Marsh"),
        (Badges::VOLCANO, "Volcano"),
        (Badges::EARTH, "Earth"),
        ];
        
        let mut collected = Vec::new();
        for (flag, name) in names.iter() {
            if b.contains(*flag) {
                collected.push(*name);
            }
        }
        
        if collected.is_empty() {
            Err(PokemonError::LookupError)
        } else {
            Ok(collected)
        }
    }
    
    pub fn get_player_id(&self) -> u16 {
        self.read_u16_be(offsets::PLAYER_ID)
    }

    // Function to ensure parameters passed to copy_party_pokemon() are valid
    // and that the copy operation will be a success prior to making any changes
    // to the save data.
    fn validate_copy_pokemon_operation(&self, party_slot: usize, box_number: usize) -> Result<(), PokemonError> {

        if party_slot <= 0 || party_slot > offsets::MAX_PARTY_SIZE {
            return Err(PokemonError::InvalidPartySlot);
        }        
        if box_number <= 0 || box_number > offsets::NUM_POKEMON_BOXES {
            return Err(PokemonError::InvalidBoxNumber);
        }

        // Putting pokemon in the current box doesn't work correctly since the game keeps current box
        // data in a temporary location and changes here will get overwritten when interacting with the PC
        if box_number == self.get_current_box() {
            return Err(PokemonError::BoxInUse);
        }

        if !self.box_has_free_slot(box_number) {
            return Err(PokemonError::PokemonBoxFull);
        }

        if !self.is_valid_party_slot(party_slot) {
            return Err(PokemonError::InvalidPartySlot);
        }


        Ok(())
    }

    // Returns true if given box number has at least one free space in it
    fn box_has_free_slot(&self, box_number: usize) -> bool {

        
        // If box number is not valid, return false
        if box_number < 1 || box_number > 12 {
            return false;
        }

        if self.get_current_box() == box_number {
            return self.get_current_box_pokemon_count() < offsets::MAX_POKEMON_BOX_SIZE;
        }
        self.get_box_pokemon_count(box_number) < 20
    }

    fn is_valid_party_slot(&self, party_slot: usize) -> bool {
        // Ensure the party slot passed in is valid and has a pokemone in it, otherwise error and abort
        party_slot <= self.get_party_count()
    }
    
    pub fn copy_party_pokemon(&mut self, party_slot: usize, box_number: usize) -> Result<(), PokemonError> {
        
        // Check parameters passed in are valid and the box 
        // has a free slot. Throw an error and abort the operation 
        // if the validation fails.
        if let Err(e) = self.validate_copy_pokemon_operation(party_slot, box_number) {
            return Err(e);
        }
                
        // Copy the pokemon's data from the party as a PokemonRaw object
        // The last line converts the pokmeon data from the 44 byte party structure to the 33 byte box structure we 
        // need to move it from party to box.
        let party_pokemon_offset = offsets::PARTY_FIRST_PKMN + (offsets::PARTY_NEXT_PKMN * (party_slot - 1));       
        let raw_pokemon = self.read_party_pokemon_raw(party_pokemon_offset);
        let data = raw_pokemon.get_for_box();
        let species_id = data[0];
        

        let mut box_base_offset ;
        // Determine detination PC box offset to write copied data. 
        // If box 1-6 (bank2), start at the begin of bank 2
        // If box 7-12 (bank3), start at the begin of bank 3.
        // then move the offset forward to the start of the selected box within bank 1 or 2.
        if box_number <= 6 {
            box_base_offset = offsets::BOX_1_DATA_OFFSET;
            box_base_offset += offsets::BOX_NEXT_BOX * (box_number - 1);
        }
        else {
            box_base_offset = offsets::BOX_7_DATA_OFFSET;
            box_base_offset += offsets::BOX_NEXT_BOX * (box_number - 7);
        }
        
        // Next we set asside the destination offsets that the pokemon's OT and nick name will be written to.
        // OT and nick name data are not stored in the main pokemon data structure and are written seperately.
        let box_count = self.get_box_pokemon_count(box_number);
        let ot_destination_offset = box_base_offset + offsets::BOX_FIRST_OT + (offsets::PARTY_OT_NICK_SIZE * box_count);
        let nick_destination_offset = box_base_offset + offsets::BOX_FIRST_NICK + (offsets::PARTY_OT_NICK_SIZE * box_count);

        // Here we copy the current pokemon's OT and nick name from party data so we can copy them to the box.
        let ot_source_offset = offsets::PARTY_FIRST_OT + (offsets::PARTY_OT_NICK_SIZE * (party_slot - 1));
        let ot_name = self.read_string(ot_source_offset, 0x50);
        let nick_source_offset = offsets::PARTY_FIRST_NICK + (offsets::PARTY_OT_NICK_SIZE * (party_slot - 1));
        let nick_name = self.read_string(nick_source_offset, offsets::NAME_TERMINATOR);
        
        // Lastly, move the current offset forward to the first empty slot in the destination box. We're ready to write the main
        // pokmeon data here next.
        box_base_offset += offsets::BOX_START_TO_FIRST;
        box_base_offset += offsets::BOX_NEXT_PKMN * box_count;
        
        // Write 33 byte pokemon structure to PC box (Main pokemon data w/o nick and OT)
        self.write_bytes(box_base_offset, &data);

        // Next we need to update the box count by 1 so the game knows we inserted a pokemon
        let count_update_offset = self.get_box_pokemon_count_offset(box_number);
        self.write_byte(count_update_offset, (box_count + 1) as u8);

        // The beginning of a pokemon list is a list of the species ID's of the pokemon in the box. Here we're
        // inserting the pokemon we added to the box's species ID to the end of that list and a list terminator character 0xFF
        let species_update_offset = count_update_offset + box_count + 1;
        let species_data = &[species_id, 0xFF];
        self.write_bytes(species_update_offset, species_data);
        
        // Finally, write the nickname and OT strings to the PC box. This data is kept seperately from the pokemon's main data
        // structure
        self.write_string(&ot_name, ot_destination_offset, offsets::NAME_TERMINATOR);
        self.write_string(&nick_name, nick_destination_offset, offsets::NAME_TERMINATOR);
        
        // Ok all finished! Remember, must call .save() on the SaveFile so all the checksums get updated!!!
        Ok(())
    }


    pub fn get_current_box(&self) -> usize {
        ((self.read_byte(offsets::CURRENT_BOX) & 0x7F) + 1) as usize
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