use std::fs;

pub struct SaveFile {
    data: Vec<u8>
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

    pub fn read_bytes(&self, start: usize, end: usize) -> &[u8] {
        &self.data[start..=end]
    }

    pub fn write_byte(&mut self, offset: usize, value: u8) {
        self.data[offset] = value;
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        fs::write(filename, &self.data)
    }
}