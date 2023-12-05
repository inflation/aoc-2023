#[must_use]
pub fn atoi(input: &[u8]) -> u32 {
    input
        .iter()
        .fold(0, |acc, &c| acc * 10 + u32::from(c - b'0'))
}
