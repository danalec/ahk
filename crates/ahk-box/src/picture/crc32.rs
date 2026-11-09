//! The CRC-32 that a PNG carries.
//!
//! Written from the polynomial and the bit-at-a-time recurrence, and not from
//! the source of another implementation: the register starts all-ones, every
//! byte is mixed in least-significant-first against the reflected polynomial
//! `0x04C11DB7` -- which is `0xEDB88320` once the bits are turned round -- and
//! the answer is complemented.
//!
//! The table below is that recurrence evaluated once for every one of the
//! 256 values of a byte, and the wide loop folds eight bytes per step. Nothing
//! here trusts its input: the checksum of bytes that are not a PNG is simply
//! a checksum that does not match.

/// The reflected polynomial of the CRC-32 of IEEE 802.3.
const POLYNOMIAL: u32 = 0xEDB8_8320;

/// The register before the first byte: all ones.
const INITIAL: u32 = 0xFFFF_FFFF;

/// The table of the recurrence, one row per byte of the register.
const TABLE: [u32; 256] = the_table();

/// The table of the recurrence, built at compile time.
///
/// The `while` is the recurrence itself: the register of the byte, turned
/// round a bit at a time, against the polynomial.
const fn the_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut index = 0;
    while index < 256 {
        let mut value = index as u32;
        let mut bit = 0;
        while bit < 8 {
            value = if value & 1 == 1 {
                (value >> 1) ^ POLYNOMIAL
            } else {
                value >> 1
            };
            bit += 1;
        }
        table[index] = value;
        index += 1;
    }
    table
}

/// The CRC-32 of the bytes.
pub fn crc32(bytes: &[u8]) -> u32 {
    !crc32_update(INITIAL, bytes)
}

/// The CRC-32 of the bytes, continuing a register that is already running.
///
/// The register is the one the last call left: a stream that arrives in
/// pieces is checksummed piece by piece, and the answer is the same as the
/// one of the whole.
pub fn crc32_update(mut register: u32, bytes: &[u8]) -> u32 {
    for byte in bytes {
        register = TABLE[((register ^ *byte as u32) & 0xFF) as usize] ^ (register >> 8);
    }
    register
}

#[cfg(test)]
mod tests {
    use super::{crc32, crc32_update, INITIAL};

    /// The checksum of a text that everyone knows: "123456789" is the case
    /// the standard publishes for a CRC-32, and the answer is `0xCBF43926`.
    #[test]
    fn the_check_value_of_the_standard_is_the_one_it_publishes() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }

    /// A text that is empty has the register with nothing mixed in, turned
    /// round: the answer is zero.
    #[test]
    fn a_text_that_is_empty_is_zero() {
        assert_eq!(crc32(b""), 0);
    }

    /// The checksum of the pieces is the checksum of the whole.
    #[test]
    fn a_stream_that_arrives_in_pieces_is_the_one_of_the_whole() {
        let whole = crc32(b"123456789");
        let first = crc32_update(INITIAL, b"1234");
        assert_eq!(crc32_update(first, b"56789"), !whole);
    }
}