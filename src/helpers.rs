use std::fs::File;
use std::io::{self, Read, BufReader};
use std::str;

use byteorder::{BigEndian, ReadBytesExt};

use utf16string::{WStr};

pub fn read_u8(reader: &mut BufReader<File>) -> u8 {
    reader.read_u8().unwrap()
}

pub fn read_bool(reader: &mut BufReader<File>) -> bool {
    match read_u8(reader) {
        0 => false,
        1 => true,
        _ => panic!("Oh no!")
    }
}

pub fn read_u16(reader: &mut BufReader<File>) -> u16 {
    reader.read_u16::<BigEndian>().unwrap()
}

pub fn read_u32(reader: &mut BufReader<File>) -> u32 {
    reader.read_u32::<BigEndian>().unwrap()
}

pub fn read_unicode_string_from_reader(reader: &mut BufReader<File>) -> io::Result<String> {
    let len = read_u32(reader) - 1;

    let mut buffer = Vec::new();
    reader.take(2 * len as u64).read_to_end(&mut buffer)?;

    let _ = read_u16(reader);
 
    // Convert the binary data to a Unicode string using the appropriate encoding

    match str::from_utf8(&buffer) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}