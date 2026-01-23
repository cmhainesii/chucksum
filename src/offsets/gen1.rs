pub const CHECKSUM_START: usize = 0x2598;
pub const CHECKSUM_END: usize  = 0x3522;
pub const CHECKSUM_OFFSET: usize = 0x3523;
pub const PLAYER_NAME_OFFSET: usize = 0x2598;
pub const RIVAL_NAME_OFFSET: usize = 0x25F6;
pub const MONEY_OFFSET: usize = 0x25F3;
pub const MONEY_MAX: u32 = 999_999;
pub const NAME_TERMINATOR: u8 = 0x50;

pub const PLAYER_ID: usize = 0x2605;

// Item list constants - GEN 1
pub const BAG_OFFSET: usize = 0x25C9; // Beginning of Bag item list data.
pub const MAX_BAG_ITEMS: usize = 20;
pub const LIST_ITEM_SIZE: usize = 2;

// This is the offset of the first item in the list relative to the list head
pub const ITEM_LIST_FIRST_ITEM: usize = 1;

// Item box constants
pub const MAX_BOX_ITEMS: usize = 50;
pub const BOX_ITEMS_OFFSET: usize = 0x27E6;

// Party related constants
pub const PARTY_DATA_OFFSET: usize = 0x2F2C; // Beginning of party data. Party count
pub const _MAX_PARTY_SIZE: usize = 6;
pub const PARTY_SPECIES_LIST_OFFSET: usize = 1; // Add this to party data offset to get first species in species list.
pub const _PARTY_LIST_TERMINATOR: u8 = 0xFF;
pub const PARTY_START_TO_FIRST: usize = 8; // Add this to party data offset to get to first party pokemon

pub const BADGES: usize = 0x2602;

pub const BOX_1_DATA_OFFSET: usize = 0x4000;
pub const BOX_6_DATA_OFFSET: usize = 0x6000;
pub const BOX_CURRENT_DATA_OFFSET: usize = 0x30C0;
pub const BOX_START_TO_FIRST: usize = 0x16;

// All adition below will be rooting from this first offset. Begin of first party pokemon data.
pub const PARTY_FIRST_PKMN: usize = PARTY_DATA_OFFSET + PARTY_START_TO_FIRST;
pub const BOX1_FIRST_PKMN: usize = BOX_1_DATA_OFFSET + BOX_START_TO_FIRST;
pub const BOX6_FIRST_PKMN: usize = BOX_6_DATA_OFFSET + BOX_START_TO_FIRST;
pub const BOX_CURRENT_FIRST_PKMN: usize = BOX_CURRENT_DATA_OFFSET + BOX_START_TO_FIRST;

// All of these constants can be added to the current pokemon's offset 
// to locate the various data within the games pokemon data structure.
// Example: (FIRST_PKMN_OFFSET + SPECIES_ID) yields the byte holding the species
// id for the first party pokemon.
pub const PARTY_SPECIES_ID: usize = 0x00;
pub const PARTY_CURRENT_HP: usize = 0x01;
pub const PARTY_LEVEL: usize = 0x03;
pub const PARTY_STATUS: usize = 0x04;
pub const PARTY_TYPE_1: usize = 0x05;
pub const PARTY_TYPE_2: usize = 0x06;
pub const PARTY_CATCH_RATE: usize = 0x07;
pub const PARTY_MOVE_INDEX_1: usize = 0x08;
pub const PARTY_MOVE_INDEX_2: usize = 0x09;
pub const PARTY_MOVE_INDEX_3: usize = 0x0A;
pub const PARTY_MOVE_INDEX_4: usize = 0x0B;
pub const PARTY_OT_ID: usize = 0x0C;
pub const PARTY_EXPERIENCE_PTS: usize = 0x0E;
pub const PARTY_HP_STAT_EXP: usize = 0x11;
pub const PARTY_ATTACK_STAT_EXP: usize = 0x13;
pub const PARTY_DEFENSE_STAT_EXP: usize = 0x15;
pub const PARTY_SPEED_STAT_EXP: usize = 0x17;
pub const PARTY_SPECIAL_STAT_EXP: usize = 0x19;
pub const PARTY_IV_1: usize = 0x1B;
pub const PARTY_IV_2: usize = 0x1C;
pub const PARTY_NEXT_PKMN: usize = 0x2C;