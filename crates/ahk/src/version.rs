//! The version of a program of Windows, read out of the file of it.
//!
//! A program of Windows carries its version in a resource of its own: the
//! `VS_VERSION_INFO`, a blob that the loader of Windows reads and that says
//! the version of the file and the do-not-modify of it. This module reads
//! that blob out of the bytes of a file.
//!
//! Measured on the binary of v2.0.19 on 2026-11-11: `FileGetVersion` of a
//! program of Windows answers the version as four numbers and a point
//! between them -- "10.0.26100.9818" for the notepad of that day -- and a
//! file that has no section of resources throws "(1812) The specified image
//! file did not contain a resource section.".
//!
//! The file is read as a program of Windows: the header of the DOS, the
//! header of the PE, the table of the sections, and the table of the
//! resources -- where the resource of the version is the one of the type
//! sixteen. The version itself is the one of the `VS_FIXEDFILEINFO`, and
//! the text of the `FileVersion` is the one to fall back on.

/// The type of the resource that carries the version of a program.
const RT_VERSION: u32 = 16;

/// The mark of the `VS_FIXEDFILEINFO`.
const THE_MARK_OF_THE_FIXED: u32 = 0xFEEF_04BD;

/// The version of a program of Windows, out of the bytes of its file.
///
/// `None` is a file that is not a program of Windows, or a program that
/// carries no version: the caller turns that into the failure of the
/// machine.
pub fn the_version_of(bytes: &[u8]) -> Option<String> {
    let blob = the_blob_of_the_version(bytes)?;
    let fixed = the_fixed_of_the_blob(&blob);
    match fixed {
        Some(version) => Some(version),
        // A program may carry the version only as a text, and then the text
        // is the version.
        None => the_text_of_the_version(&blob),
    }
}

/// The blob of the version, out of the bytes of a program.
fn the_blob_of_the_version(bytes: &[u8]) -> Option<Vec<u8>> {
    // The header of the DOS, and the place of the header of the PE.
    if bytes.len() < 0x40 || &bytes[0..2] != b"MZ" {
        return None;
    }
    let header = the_u32(bytes, 0x3C)? as usize;
    if bytes.len() < header + 24 || &bytes[header..header + 4] != b"PE\0\0" {
        return None;
    }
    let sections = the_u16(bytes, header + 6)? as usize;
    let size_of_the_optional = the_u16(bytes, header + 20)? as usize;
    let optional = header + 24;
    let magic = the_u16(bytes, optional)?;
    // The table of the directories of the optional header: the resource is
    // the third of them, and the place of the table is not the same in the
    // two kinds of program.
    let directories = match magic {
        0x10B => optional + 96,
        0x20B => optional + 112,
        _ => return None,
    };
    let resource = the_u32(bytes, directories + 16)?;
    let size = the_u32(bytes, directories + 20)? as usize;
    if resource == 0 || size == 0 {
        return None;
    }
    // The table of the sections, and the place of a section in the file.
    let table = optional + size_of_the_optional;
    let the_file_place = |place: u32| -> Option<usize> {
        for number in 0..sections {
            let row = table + number * 40;
            let address = the_u32(bytes, row + 12)?;
            // The section covers the addresses between its own address and
            // the end of it -- and the size of it in the file and the size of
            // it in the memory are two sizes, and the loader takes the
            // greater of the two.
            let virtual_size = the_u32(bytes, row + 8)?;
            let raw_size = the_u32(bytes, row + 16)?;
            let size = virtual_size.max(raw_size);
            let raw = the_u32(bytes, row + 20)?;
            if place >= address && place < address + size {
                return Some((raw + (place - address)) as usize);
            }
        }
        None
    };
    let root = the_file_place(resource)?;
    // The table of the resources: the first level is the kind of the
    // resource, and the version is the one of the kind sixteen.
    let leaf = the_entry_of_the_kind(bytes, root, RT_VERSION)?;
    // The leaf carries the place of the blob and its size: the place is an
    // address of the program, and the file is read where it says.
    let rva = the_u32(bytes, leaf)?;
    let size = the_u32(bytes, leaf + 4)? as usize;
    let blob = the_file_place(rva)?;
    bytes.get(blob..blob + size).map(|slice| slice.to_vec())
}

/// The place of the entry of a kind of resource.
///
/// The table of the resources is a tree of three levels: the kind, the name,
/// and the language. The version is the kind sixteen, and the leaf of it is
/// what carries the blob.
fn the_entry_of_the_kind(bytes: &[u8], root: usize, kind: u32) -> Option<usize> {
    let entries = the_u16(bytes, root + 12)? as usize + the_u16(bytes, root + 14)? as usize;
    for number in 0..entries {
        let entry = root + 16 + number * 8;
        let name = the_u32(bytes, entry)?;
        // The name of a kind is a number, and the high bit of it says that
        // the name is a text instead.
        if name & 0x8000_0000 != 0 || name != kind {
            continue;
        }
        let offset = the_u32(bytes, entry + 4)?;
        // The high bit of the offset says that the entry is a directory and
        // not the leaf.
        if offset & 0x8000_0000 == 0 {
            return Some(root + (offset & 0x7FFF_FFFF) as usize);
        }
        let directory = root + (offset & 0x7FFF_FFFF) as usize;
        // The second level is the name of the resource, and the third is the
        // language: the first of each is the one that is wanted.
        let names = the_u16(bytes, directory + 12)? as usize + the_u16(bytes, directory + 14)? as usize;
        if names == 0 {
            return None;
        }
        let name_entry = directory + 16;
        let name_offset = the_u32(bytes, name_entry + 4)?;
        if name_offset & 0x8000_0000 == 0 {
            return Some(root + (name_offset & 0x7FFF_FFFF) as usize);
        }
        let languages = root + (name_offset & 0x7FFF_FFFF) as usize;
        let count = the_u16(bytes, languages + 12)? as usize + the_u16(bytes, languages + 14)? as usize;
        if count == 0 {
            return None;
        }
        let language = languages + 16;
        let leaf = the_u32(bytes, language + 4)?;
        if leaf & 0x8000_0000 != 0 {
            return None;
        }
        // The place of the leaf is a place in the table of the resources.
        return Some(root + leaf as usize);
    }
    None
}

/// The version of the `VS_FIXEDFILEINFO`, as four numbers.
///
/// The version of a program is in the high and the low halves of two
/// numbers of thirty-two bits, and each half carries two numbers of
/// sixteen bits.
fn the_fixed_of_the_blob(blob: &[u8]) -> Option<String> {
    // The fixed part carries its own mark, and the mark is what says that
    // the bytes that follow it are the version: the place of it inside the
    // blob depends on the padding of the key, so it is looked for.
    let mut at = 0;
    while at + 52 <= blob.len() {
        if the_u32(blob, at) == Some(THE_MARK_OF_THE_FIXED) {
            let high = the_u32(blob, at + 8)?;
            let low = the_u32(blob, at + 12)?;
            return Some(format!(
                "{}.{}.{}.{}",
                high >> 16,
                high & 0xFFFF,
                low >> 16,
                low & 0xFFFF
            ));
        }
        at += 4;
    }
    None
}

/// The version of a program, as the text of its `FileVersion`.
///
/// A program may carry no fixed part, and then the version is the text
/// that the table of the strings holds.
fn the_text_of_the_version(blob: &[u8]) -> Option<String> {
    let text = the_utf16_of(blob);
    let key = "FileVersion";
    let mut at = 0;
    while let Some(found) = text[at..].find(key) {
        let start = at + found + key.len();
        let value: String = text[start..]
            .chars()
            .take_while(|letter| *letter != '\0')
            .collect();
        let value = value.trim_start_matches(' ').trim();
        if !value.is_empty() {
            return Some(value.to_string());
        }
        at = start;
    }
    None
}

/// The bytes of a blob, as the text of them.
fn the_utf16_of(blob: &[u8]) -> String {
    let mut letters = Vec::new();
    let mut at = 0;
    while at + 1 < blob.len() {
        letters.push(u16::from_le_bytes([blob[at], blob[at + 1]]));
        at += 2;
    }
    String::from_utf16_lossy(&letters)
}

/// A byte of the file, as a number of sixteen bits.
fn the_u16(bytes: &[u8], at: usize) -> Option<u16> {
    let slice = bytes.get(at..at + 2)?;
    Some(u16::from_le_bytes([slice[0], slice[1]]))
}

/// A byte of the file, as a number of thirty-two bits.
fn the_u32(bytes: &[u8], at: usize) -> Option<u32> {
    let slice = bytes.get(at..at + 4)?;
    Some(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

#[cfg(test)]
mod tests {
    use super::the_version_of;

    /// A program of Windows with a version in it, made here.
    ///
    /// The file is the smallest one that carries a version: the header of
    /// the DOS, the header of the PE, one section, and the table of the
    /// resources with the resource of the version in it.
    fn a_program(version: (u16, u16, u16, u16)) -> Vec<u8> {
        let mut file = vec![0u8; 0x600];
        // The header of the DOS.
        file[0] = b'M';
        file[1] = b'Z';
        file[0x3C] = 0x80;
        // The header of the PE.
        let header = 0x80;
        file[header..header + 4].copy_from_slice(b"PE\0\0");
        file[header + 6..header + 8].copy_from_slice(&1u16.to_le_bytes()); // one section
        file[header + 20..header + 22].copy_from_slice(&0xE0u16.to_le_bytes()); // the size of the optional header
        // The optional header: the magic of a program of thirty-two bits, and
        // the table of the directories -- the resource is the third of them.
        let optional = header + 24;
        file[optional..optional + 2].copy_from_slice(&0x10Bu16.to_le_bytes());
        let directories = optional + 96;
        // The resource is at the place 0x400 of the file, and it is the third
        // directory.
        file[directories + 16..directories + 20].copy_from_slice(&0x400u32.to_le_bytes());
        file[directories + 20..directories + 24].copy_from_slice(&0x200u32.to_le_bytes());
        // The section that carries the resource.
        let table = optional + 0xE0;
        file[table..table + 8].copy_from_slice(b".rsrc\0\0\0");
        file[table + 12..table + 16].copy_from_slice(&0x400u32.to_le_bytes()); // the address of the section
        file[table + 16..table + 20].copy_from_slice(&0x200u32.to_le_bytes()); // the size of the raw data
        file[table + 20..table + 24].copy_from_slice(&0x400u32.to_le_bytes()); // the place of the raw data
        // The table of the resources: the kind of the resource is the first
        // entry, and it says sixteen -- the version.
        let root = 0x400;
        file[root + 12..root + 14].copy_from_slice(&1u16.to_le_bytes()); // one entry by number
        file[root + 16..root + 20].copy_from_slice(&16u32.to_le_bytes()); // the kind
        file[root + 20..root + 24].copy_from_slice(&0x8000_0020u32.to_le_bytes()); // the directory of the names
        // The second level: the name of the resource, and the third: the
        // language, and the leaf that carries the blob.
        let names = root + 0x20;
        file[names + 12..names + 14].copy_from_slice(&1u16.to_le_bytes());
        file[names + 16..names + 20].copy_from_slice(&1u32.to_le_bytes());
        file[names + 20..names + 24].copy_from_slice(&0x8000_0040u32.to_le_bytes());
        let languages = root + 0x40;
        file[languages + 12..languages + 14].copy_from_slice(&1u16.to_le_bytes());
        file[languages + 16..languages + 20].copy_from_slice(&0x409u32.to_le_bytes());
        file[languages + 20..languages + 24].copy_from_slice(&0x60u32.to_le_bytes());
        // The leaf: the place of the blob and its size.
        let leaf = root + 0x60;
        file[leaf..leaf + 4].copy_from_slice(&0x480u32.to_le_bytes());
        file[leaf + 4..leaf + 8].copy_from_slice(&0x100u32.to_le_bytes());
        // The blob of the version.
        let blob = 0x480;
        let key: Vec<u16> = "VS_VERSION_INFO".encode_utf16().chain([0]).collect();
        let mut at = blob;
        file[at..at + 2].copy_from_slice(&0x100u16.to_le_bytes()); // the length of the blob
        file[at + 2..at + 4].copy_from_slice(&52u16.to_le_bytes()); // the length of the value
        for letter in key {
            file[at..at + 2].copy_from_slice(&letter.to_le_bytes());
            at += 2;
        }
        at = (at + 3) & !3;
        // The fixed part.
        file[at..at + 4].copy_from_slice(&0xFEEF_04BDu32.to_le_bytes());
        let high = ((version.0 as u32) << 16) | version.1 as u32;
        let low = ((version.2 as u32) << 16) | version.3 as u32;
        file[at + 8..at + 12].copy_from_slice(&high.to_le_bytes());
        file[at + 12..at + 16].copy_from_slice(&low.to_le_bytes());
        file
    }

    #[test]
    fn the_version_of_a_program_is_the_four_numbers_of_it() {
        let file = a_program((10, 0, 26100, 9818));
        assert_eq!(the_version_of(&file), Some("10.0.26100.9818".to_string()));
    }

    #[test]
    fn a_file_that_is_not_a_program_of_windows_has_no_version() {
        assert_eq!(the_version_of(b"not a program at all"), None);
        // The header of the DOS without the header of the PE behind it.
        let mut file = a_program((1, 0, 0, 0));
        file[0x80] = 0;
        assert_eq!(the_version_of(&file), None);
    }

    #[test]
    fn a_program_without_a_section_of_resources_has_no_version() {
        let mut file = a_program((1, 0, 0, 0));
        // The resource directory of the optional header is empty.
        let directories = 0x80 + 24 + 96;
        file[directories + 16..directories + 20].copy_from_slice(&0u32.to_le_bytes());
        assert_eq!(the_version_of(&file), None);
    }
}
