use std::fs;

pub struct SaveFile {
    data: Vec<u8>
}

impl SaveFile {

    const GEN1_CHECKSUM_START: usize = 0x2598;
    const GEN1_CHECKSUM_END: usize  = 0x3522;
    const GEN1_CHECKSUM_OFFSET: usize = 0x3523;

    pub fn new(filename: &str) -> std::io::Result<Self> {

        let data = fs::read(filename)?;
        Ok(Self { data })
    }

    pub fn len(&self) -> usize { self.data.len() }

    pub fn read_byte(&self, offset: usize) -> u8 {
        self.data[offset]
    }

    pub fn read_bytes(&self, start: usize, end: usize) -> &[u8] {
        &self.data[start..=end]
    }

    pub fn write_byte(&mut self, offset: usize, value: u8) {
        self.data[offset] = value;
    }

    pub fn as_slice(&self) -> &[u8] {
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
}