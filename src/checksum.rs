pub fn calculate_checksum(data: &[u8], start: usize, end: usize) -> u8 {
    let mut checksum: u8 = 0;

    for byte in &data[start..=end] {
        checksum = checksum.wrapping_add(*byte);
    }
    !checksum
}

pub fn validate_checksum(data: &[u8], calculated: u8, checksum_offset: usize) -> bool {
    let stored_checksum = data[checksum_offset];
    stored_checksum == calculated    
}