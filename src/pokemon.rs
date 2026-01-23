use core::fmt;

use crate::offsets;

fn get_high_nibble(b: u8) -> u8 {
    (b >> 4) & 0x0F
}

fn get_low_nibble(b: u8) -> u8 {
    b & 0x0F
}
pub struct Pokemon {
    pub species_id: u8,
    pub current_hp: u16,
    pub level: u8,
    pub status: u8,
    pub pkmn_type_1: u8,
    pub pkmn_type_2: u8,
    pub catch_rate: u8,
    pub move_index1: u8,
    pub move_index2: u8,
    pub move_index3: u8,
    pub move_index4: u8,
    pub ot_id: u16,
    pub experience_pts: u32,
    pub hp_stat_exp: u16,
    pub attack_stat_exp: u16,
    pub defense_stat_exp: u16,
    pub speed_stat_exp: u16,
    pub special_stat_exp: u16,
    pub attack_iv: u8,
    pub defense_iv: u8,
    pub speed_iv: u8,
    pub special_iv: u8,
}

pub struct PokemonRaw {
    data: [u8; offsets::PARTY_NEXT_PKMN]
}

impl PokemonRaw {

    pub fn new(data: [u8; offsets::PARTY_NEXT_PKMN]) -> Self {
        PokemonRaw { data }
    }

    fn byte(&self, offset: usize) -> u8 {
        return self.data[offset]
    }

    fn u16_be(&self, offset: usize) -> u16 {
        u16::from_be_bytes([self.data[offset], self.data[offset + 1]])
    }

    fn u16_le(&self, offset: usize) -> u16 {
        u16::from_le_bytes([self.data[offset], self.data[offset + 1]])
    }

    fn u24_be(&self, offset: usize) -> u32 {
        ((self.data[offset] as u32) << 16)
            | ((self.data[offset+1] as u32) << 8)
            | self.data[offset + 2] as u32
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StatusCondtion {
    None,
    Asleep,
    Poisoned,
    Burned,
    Frozen,
    Paralyzed,
}

impl fmt::Display for StatusCondtion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            StatusCondtion::None => "None",
            StatusCondtion::Asleep => "Asleep",
            StatusCondtion::Poisoned => "Poisoned",
            StatusCondtion::Burned => "Burned",
            StatusCondtion::Frozen => "Frozen",
            StatusCondtion::Paralyzed => "Paralyzed",
        };

        write!(f, "{s}")
    }
}

impl StatusCondtion {
    pub fn from_byte(value: u8) -> Self {
        match value {
            0x04 => StatusCondtion::Asleep,
            0x08 => StatusCondtion::Poisoned,
            0x10 => StatusCondtion::Burned,
            0x20 => StatusCondtion::Frozen,
            0x40 => StatusCondtion::Paralyzed,
            _ => StatusCondtion::None,
        }
    }
}

impl Pokemon {

    

    pub fn get_type_name(pkmn_type: u8) -> &'static str {
        match pkmn_type {
            0  => "Normal",
            1  => "Fighting",
            2  => "Flying",
            3  => "Poison",
            4  => "Ground",
            5  => "Rock",
            7  => "Bug",
            8  => "Ghost",
            20 => "Fire",
            21 => "Water",
            22 => "Grass",
            23 => "Electric",
            24 => "Psychic",
            25 => "Ice",
            26 => "Dragon",
            _  => "Invalid/Unknown"
        }
    }






    pub fn from_raw(raw: PokemonRaw) -> Pokemon {
        let iv1 = raw.byte(offsets::PARTY_IV_1);
        let iv2 = raw.byte(offsets::PARTY_IV_2);

        let attack_iv = get_high_nibble(iv1);
        let defense_iv = get_low_nibble(iv1);
        let speed_iv = get_high_nibble(iv2);
        let special_iv = get_low_nibble(iv2);

        Pokemon {
            species_id: raw.byte(offsets::PARTY_SPECIES_ID),
            current_hp: raw.u16_be(offsets::PARTY_CURRENT_HP),
            level: raw.byte(offsets::PARTY_LEVEL),
            status: raw.byte(offsets::PARTY_STATUS),
            pkmn_type_1: raw.byte(offsets::PARTY_TYPE_1),
            pkmn_type_2: raw.byte(offsets::PARTY_TYPE_2),
            catch_rate: raw.byte(offsets::PARTY_CATCH_RATE),
            move_index1: raw.byte(offsets::PARTY_MOVE_INDEX_1),
            move_index2: raw.byte(offsets::PARTY_MOVE_INDEX_2),
            move_index3: raw.byte(offsets::PARTY_MOVE_INDEX_3),
            move_index4: raw.byte(offsets::PARTY_MOVE_INDEX_4),
            ot_id: raw.u16_be(offsets::PARTY_OT_ID),
            experience_pts: raw.u24_be(offsets::PARTY_EXPERIENCE_PTS),
            hp_stat_exp: raw.u16_le(offsets::PARTY_HP_STAT_EXP),
            attack_stat_exp: raw.u16_le(offsets::PARTY_ATTACK_STAT_EXP),
            defense_stat_exp: raw.u16_le(offsets::PARTY_DEFENSE_STAT_EXP),
            speed_stat_exp: raw.u16_le(offsets::PARTY_SPEED_STAT_EXP),
            special_stat_exp: raw.u16_le(offsets::PARTY_SPECIAL_STAT_EXP),
            attack_iv, defense_iv, speed_iv, special_iv
        }
    }

    
    // Function to map pokemon species to ids
    pub fn get_species_name(id: u8) -> &'static str {

        const INVALID_SPECIES_NAME: &str = "Invalid/Unknown Species";

        match id {
            1   => "Rhydon",
            2   => "Kangaskhan",
            3   => "Nidoran♂",
            4   => "Clefairy",
            5   => "Spearow",
            6   => "Voltorb",
            7   => "Nidoking",
            8   => "Slowbro",
            9   => "Ivysaur",
            10  => "Exeggutor",
            11  => "Lickitung",
            12  => "Exeggcute",
            13  => "Grimer",
            14  => "Gengar",
            15  => "Nidoran♀",
            16  => "Nidoqueen",
            17  => "Cubone",
            18  => "Rhyhorn",
            19  => "Lapras",
            20  => "Arcanine",
            21  => "Mew",
            22  => "Gyarados",
            23  => "Shellder",
            24  => "Tentacool",
            25  => "Gastly",
            26  => "Scyther",
            27  => "Staryu",
            28  => "Blastoise",
            29  => "Pinsir",
            30  => "Tangela",
            31  => "MissingNo",
            32  => "MissingNo",
            33  => "Growlithe",
            34  => "Onix",
            35  => "Fearow",
            36  => "Pidgey",
            37  => "Slowpoke",
            38  => "Kadabra",
            39  => "Graveler",
            40  => "Chansey",
            41  => "Machoke",
            42  => "Mr. Mime",
            43  => "Hitmonlee",
            44  => "Hitmonchan",
            45  => "Arbok",
            46  => "Parasect",
            47  => "Psyduck",
            48  => "Drowzee",
            49  => "Golem",
            50  => "MissingNo",
            51  => "Magmar",
            52  => "MissingNo",
            53  => "Electabuzz",
            54  => "Magneton",
            55  => "Koffing",
            56  => "MissingNo",
            57  => "Mankey",
            58  => "Seel",
            59  => "Diglett",
            60  => "Tauros",
            61  => "MissingNo",
            62  => "MissingNo",
            63  => "MissingNo",
            64  => "Farfetch'd",
            65  => "Venonat",
            66  => "Dragonite",
            67  => "MissingNo",
            68  => "MissingNo",
            69  => "MissingNo",
            70  => "Doduo",
            71  => "Poliwag",
            72  => "Jynx",
            73  => "Moltres",
            74  => "Articuno",
            75  => "Zapdos",
            76  => "Ditto",
            77  => "Meowth",
            78  => "Krabby",
            79  => "MissingNo",
            80  => "MissingNo",
            81  => "MissingNo",
            82  => "Vulpix",
            83  => "Ninetales",
            84  => "Pikachu",
            85  => "Raichu",
            86  => "MissingNo",
            87  => "MissingNo",
            88  => "Dratini",
            89  => "Dragonair",
            90  => "Kabuto",
            91  => "Kabutops",
            92  => "Horsea",
            93  => "Seadra",
            94  => "MissingNo",
            95  => "MissingNo",
            96  => "Sandshrew",
            97  => "Sandslash",
            98  => "Omanyte",
            99  => "Omastar",
            100 => "Jigglypuff",
            101 => "Wigglytuff",
            102 => "Eevee",
            103 => "Flareon",
            104 => "Jolteon",
            105 => "Vaporeon",
            106 => "Machop",
            107 => "Zubat",
            108 => "Ekans",
            109 => "Paras",
            110 => "Poliwhirl",
            111 => "Poliwrath",
            112 => "Weedle",
            113 => "Kakuna",
            114 => "Beedrill",
            115 => "MissingNo",
            116 => "Dodrio",
            117 => "Primape",
            118 => "Dugtrio",
            119 => "Venomoth",
            120 => "Dewgong",
            121 => "MissingNo",
            122 => "MissingNo",
            123 => "Caterpie",
            124 => "Metapod",
            125 => "Butterfree",
            126 => "Machamp",
            127 => "MissingNo",
            128 => "Golduck",
            129 => "Hypno",
            130 => "Golbat",
            131 => "Mewtwo",
            132 => "Snorlax",
            133 => "Magikarp",
            134 => "MissingNo",
            135 => "MissingNo",
            136 => "Muk",
            137 => "MissingNo",
            138 => "Kingler",
            139 => "Cloyster",
            140 => "MissingNo",
            141 => "Electrode",
            142 => "Clefable",
            143 => "Weezing",
            144 => "Persian",
            145 => "Marowak",
            146 => "MissingNo",
            147 => "Haunter",
            148 => "Abra",
            149 => "Alakazam",
            150 => "Pidgeotto",
            151 => "Pidgeot",
            152 => "Starmie",
            153 => "Bulbasaur",
            154 => "Venusaur",
            155 => "Tentacruel",
            156 => "MissingNo",
            157 => "Goldeen",
            158 => "Seaking",
            159 => "MissingNo",
            160 => "MissingNo",
            161 => "MissingNo",
            162 => "MissingNo",
            163 => "Ponyta",
            164 => "Rapidash",
            165 => "Rattata",
            166 => "Raticate",
            167 => "Nidorino",
            168 => "Nidorina",
            169 => "Geodude",
            170 => "Porygon",
            171 => "Aerodactyl",
            172 => "MissingNo",
            173 => "Magnemite",
            174 => "MissingNo",
            175 => "MissingNo",
            176 => "Charmander",
            177 => "Squirtle",
            178 => "Charmeleon",
            179 => "Wartortle",
            180 => "Charizard",
            181 => "MissingNo",
            182 => "MissingNo",
            183 => "MissingNo",
            184 => "MissingNo",
            185 => "Oddish",
            186 => "Gloom",
            187 => "Vileplume",
            188 => "Bellsprout",
            189 => "Weepinbell",
            190 => "Victreebel",
            _ => INVALID_SPECIES_NAME
        }
    }

    pub fn get_move_name(id: u8) -> &'static str {
        match id {
            0   => "<None>",
            1   => "Pound",
            2   => "Karate Chop",
            3   => "Double Slap",
            4   => "Comet Punch",
            6   => "Mega Punch",
            7   => "Fire Punch",
            8   => "Ice Punch",
            9   => "Thunder Punch",
            10  => "Scratch",
            11  => "Vise Grip",
            12  => "Guillotine",
            13  => "Razor Wind",
            14  => "Swords Dance",
            15  => "Cut",
            16  => "Gust",
            17  => "Wing Attack",
            18  => "Whirlwind",
            19  => "Fly",
            20  => "Bind",
            21  => "Slam",
            22  => "Vine Whip",
            23  => "Stomp",
            24  => "Double Kick",
            25  => "Mega Kick",
            26  => "Jump Kick",
            27  => "Rolling Kick",
            28  => "Sand Attack",
            29  => "Headbutt",
            30  => "Horn Attack",
            31  => "Fury Attack",
            32  => "Horn Drill",
            33  => "Tackle",
            34  => "Body Slam",
            35  => "Wrap",
            36  => "Take Down",
            37  => "Thrash",
            38  => "Double Edge",
            39  => "Tail Whip",
            40  => "Poison Sting",
            41  => "Twineedle",
            42  => "Pin Missile",
            43  => "Leer",
            44  => "Bite",
            45  => "Growl",
            46  => "Roar",
            47  => "Sing",
            48  => "Supersonic",
            49  => "Sonic Boom",
            50  => "Disable",
            51  => "Acid",
            52  => "Ember",
            53  => "Flamethrower",
            54  => "Mist",
            55  => "Water Gun",
            56  => "Hydro Pump",
            57  => "Surf",
            58  => "Ice Beam",
            59  => "Blizzard",
            60  => "Psybeam",
            61  => "Bubble Beam",
            62  => "Aurora Beam",
            63  => "Hyper Beam",
            64  => "Peck",
            65  => "Drill Peck",
            66  => "Submission",
            67  => "Low Kick",
            68  => "Counter",
            69  => "Seismic Toss",
            70  => "Strength",
            71  => "Absorb",
            72  => "Mega Drain",
            73  => "Leech Seed",
            74  => "Growth",
            75  => "Razor Leaf",
            76  => "Solar Beam",
            77  => "Poison Powder",
            78  => "Stun Spore",
            79  => "Sleep Powder",
            80  => "Petal Dance",
            81  => "String Shot",
            82  => "Dragon Rage",
            83  => "Fire Spin",
            84  => "Thunder Shock",
            85  => "Thunderbolt",
            86  => "Thunder Wave",
            87  => "Thunder",
            88  => "Rock Throw",
            89  => "Earthquake",
            90  => "Fissure",
            91  => "Dig",
            92  => "Toxic",
            93  => "Confusion",
            94  => "Psychic",
            95  => "Hypnosis",
            96  => "Meditate",
            97  => "Agility",
            98  => "Quick Attack",
            99  => "Rage",
            100 => "Teleport",
            101 => "Night Shade",
            102 => "Mimic",
            103 => "Screech",
            104 => "Double Team",
            105 => "Recover",
            106 => "Harden",
            107 => "Minimize",
            108 => "Smokescreen",
            109 => "Confuse Ray",
            110 => "Withdraw",
            111 => "Defense Curl",
            112 => "Barrier",
            113 => "Light Screen",
            114 => "Haze",
            115 => "Reflect",
            116 => "Focus Energy",
            117 => "Bide",
            118 => "Metronome",
            119 => "Mirror Move",
            120 => "Self-Destruct",
            121 => "Egg Bomb",
            122 => "Lick",
            123 => "Smog",
            124 => "Sludge",
            125 => "Bone Club",
            126 => "Fire Blast",
            127 => "Waterfall",
            128 => "Clamp",
            129 => "Swift",
            130 => "Skull Bash",
            131 => "Spike Cannon",
            132 => "Constrict",
            133 => "Amnesia",
            134 => "Kinesis",
            135 => "Soft-Boiled",
            136 => "High Jump Kick",
            137 => "Glare",
            138 => "Dream Eater",
            139 => "Poison Gas",
            140 => "Barrage",
            141 => "Leech Life",
            142 => "Lovely Kiss",
            143 => "Sky Attack",
            144 => "Transform",
            145 => "Bubble",
            146 => "Dizzy Punch",
            147 => "Spore",
            148 => "Flash",
            149 => "Psywave",
            150 => "Splash",
            151 => "Acid Armor",
            152 => "Crabhammer",
            153 => "Explosion",
            154 => "Fury Swipes",
            155 => "Bonemerang",
            156 => "Rest",
            157 => "Rock Slide",
            158 => "Hyper Fang",
            159 => "Sharpen",
            160 => "Conversion",
            161 => "Tri Attack",
            162 => "Super Fang",
            163 => "Slash",
            164 => "Substitute",
            165 => "Struggle",
            _ => "Invalid/Unknown"

        }
    }
}

