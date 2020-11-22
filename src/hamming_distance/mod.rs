/// Calculate Hamming distance to two unsigned intergers.
// ANCHOR: bit
pub fn hamming_distance(source: u64, target: u64) -> u32 {
    let mut count = 0;
    let mut xor = source ^ target; // 1

    // 2
    while xor != 0 {
        count += xor & 1; // 3
        xor >>= 1; // 4
    }

    count as u32
}
// ANCHOR_END: bit

/// Calculate Hamming distance of two UTF-8 encoded strings.
// ANCHOR: str
pub fn hamming_distance_str(source: &str, target: &str) -> usize {
    let mut count = 0;
    // 1
    let mut source = source.chars();
    let mut target = target.chars();

    loop {
        // 2
        match (source.next(), target.next()) {
            // 3
            (Some(c1), Some(c2)) if c1 != c2 => count += 1,
            // 4
            (None, Some(_)) | (Some(_), None) => panic!("Must have the same length"),
            // 5
            (None, None) => break,
            // 6
            _ => continue,
        }
    }

    count
}
// ANCHOR_END: str

#[cfg(test)]
mod base {
    use super::*;

    #[test]
    fn bit() {
        let cases = [
            (0, 0b0000_0000, 0b0000_0000),
            (0, 0b1111_1111, 0b1111_1111),
            (1, 0b0000_0001, 0b0000_0000),
            (2, 0b0000_0000, 0b0001_1000),
            (4, 0b1100_0011, 0b0110_0110),
            (8, 0b0101_0101, 0b1010_1010),
        ];
        for &(dist, c1, c2) in &cases {
            assert_eq!(hamming_distance(c1, c2), dist);
        }
    }

    #[test]
    fn str() {
        let cases = [
            (0, "", ""),
            (0, "rust", "rust"),
            (1, "cat", "bat"),
            (3, "abc", "xyz"),
        ];
        for &(dist, c1, c2) in &cases {
            assert_eq!(hamming_distance_str(c1, c2), dist);
        }
    }

    #[test]
    #[should_panic(expected = "Must have the same length")]
    fn str_panic() {
        hamming_distance_str("abc", "z");
    }
}
