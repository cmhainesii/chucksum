use chucksum::pokemon::Pokemon;
use chucksum::pokemon::PokemonRaw;
use chucksum::offsets;

#[test]
fn iv_nibbles_are_split_correctly() {
    let mut bytes = [0u8; offsets::NEXT_PARTY_PKMN];
    bytes[offsets::IV_1] = 0xAB;
    bytes[offsets::IV_2] = 0xCD;

    let p = Pokemon::from_raw(PokemonRaw::new(bytes));

    assert_eq!(p.attack_iv, 0x0A);
    assert_eq!(p.defense_iv, 0x0B);
    assert_eq!(p.speed_iv, 0x0C);
    assert_eq!(p.special_iv, 0x0D); 
}

#[test]
fn from_raw_parses_all_fields_and_endianness() {
    let mut bytes = [0u8; offsets::NEXT_PARTY_PKMN];
    bytes[offsets::SPECIES_ID] = 176; // Charmander
    bytes[offsets::CURRENT_HP] = 0x12;
    bytes[offsets::CURRENT_HP + 1] = 0x34;
    bytes[offsets::LEVEL] = 42;
    bytes[offsets::STATUS] = 0x40; // Paralyzed
    bytes[offsets::TYPE_1] = 20; // Fire
    bytes[offsets::TYPE_2] = 0; // Normal
    bytes[offsets::CATCH_RATE] = 200;
    bytes[offsets::MOVE_INDEX_1] = 52; // Ember
    bytes[offsets::MOVE_INDEX_2] = 53; //Flamethrower
    bytes[offsets::MOVE_INDEX_3] = 91; // Dig
    bytes[offsets::MOVE_INDEX_4] = 84; // Thunder Shock
    bytes[offsets::OT_ID] = 0xAA;
    bytes[offsets::OT_ID + 1] = 0xBB;
    bytes[offsets::EXPERIENCE_PTS] = 0x00;
    bytes[offsets::EXPERIENCE_PTS + 1] = 0x10;
    bytes[offsets::EXPERIENCE_PTS + 2] = 0x20;
    bytes[offsets::HP_STAT_EXP] = 0x34; // Little-endian
    bytes[offsets::HP_STAT_EXP + 1] = 0x12;
    bytes[offsets::ATTACK_STAT_EXP] = 0x78;
    bytes[offsets::ATTACK_STAT_EXP + 1] = 0x56;
    bytes[offsets::DEFENSE_STAT_EXP] = 0xBC;
    bytes[offsets::DEFENSE_STAT_EXP + 1] = 0x9A;
    bytes[offsets::SPEED_STAT_EXP] = 0xF0;
    bytes[offsets::SPEED_STAT_EXP + 1] = 0xDE;
    bytes[offsets::SPECIAL_STAT_EXP] = 0x44;
    bytes[offsets::SPECIAL_STAT_EXP + 1] = 0x33;
    bytes[offsets::IV_1] = 0xAB;
    bytes[offsets::IV_2] = 0xCD;

    let p = Pokemon::from_raw(PokemonRaw::new(bytes));

    assert_eq!(p.species_id, 176);
    assert_eq!(p.current_hp, 0x1234);
    assert_eq!(p.level, 42);
    assert_eq!(p.status, 0x40);
    assert_eq!(p.pkmn_type_1, 20);
    assert_eq!(p.pkmn_type_2, 0);
    assert_eq!(p.catch_rate, 200);
    assert_eq!(p.move_index1, 52);
    assert_eq!(p.move_index2, 53);
    assert_eq!(p.move_index3, 91);
    assert_eq!(p.move_index4, 84);
    assert_eq!(p.ot_id, 0xAABB);
    assert_eq!(p.experience_pts, 0x001020);
    assert_eq!(p.hp_stat_exp, 0x1234);
    assert_eq!(p.attack_stat_exp, 0x5678);
    assert_eq!(p.defense_stat_exp, 0x9ABC);
    assert_eq!(p.speed_stat_exp, 0xDEF0);
    assert_eq!(p.special_stat_exp, 0x3344);
    assert_eq!(p.attack_iv, 0x0A);
    assert_eq!(p.defense_iv, 0x0B);
    assert_eq!(p.speed_iv, 0x0C);
    assert_eq!(p.special_iv, 0x0D);


}

#[test]
fn get_type_name_returns_correct_name() {
    assert_eq!(Pokemon::get_type_name(0), "Normal");
    assert_eq!(Pokemon::get_type_name(1), "Fighting");
    assert_eq!(Pokemon::get_type_name(2), "Flying");
    assert_eq!(Pokemon::get_type_name(3), "Poison");
    assert_eq!(Pokemon::get_type_name(4), "Ground");
    assert_eq!(Pokemon::get_type_name(5), "Rock");
    assert_eq!(Pokemon::get_type_name(6), "Invalid/Unknown");
    assert_eq!(Pokemon::get_type_name(7), "Bug");
    assert_eq!(Pokemon::get_type_name(8), "Ghost");
    assert_eq!(Pokemon::get_type_name(20), "Fire");
    assert_eq!(Pokemon::get_type_name(21), "Water");
    assert_eq!(Pokemon::get_type_name(22), "Grass");
    assert_eq!(Pokemon::get_type_name(23), "Electric");
    assert_eq!(Pokemon::get_type_name(24), "Psychic");
    assert_eq!(Pokemon::get_type_name(25), "Ice");
    assert_eq!(Pokemon::get_type_name(26), "Dragon");
    assert_eq!(Pokemon::get_type_name(35), "Invalid/Unknown");
    assert_eq!(Pokemon::get_type_name(99), "Invalid/Unknown");
}
