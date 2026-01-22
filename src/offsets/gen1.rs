pub const CHECKSUM_START: usize = 0x2598;
pub const CHECKSUM_END: usize  = 0x3522;
pub const CHECKSUM_OFFSET: usize = 0x3523;
pub const PLAYER_NAME_OFFSET: usize = 0x2598;
pub const RIVAL_NAME_OFFSET: usize = 0x25F6;
pub const MONEY_OFFSET: usize = 0x25F3;
pub const MONEY_MAX: u32 = 999_999;
pub const NAME_TERMINATOR: u8 = 0x50;
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