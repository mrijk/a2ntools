use std::fs::File;
use std::io::{self, Read, BufReader};
use std::str;

use byteorder::{BigEndian, ReadBytesExt};

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

pub fn read_four_byte_string(reader: &mut BufReader<File>) -> io::Result<String> {
    let mut buffer = Vec::new();
    reader.take(4 as u64).read_to_end(&mut buffer)?;

    match str::from_utf8(&buffer) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}

pub fn read_string(reader: &mut BufReader<File>) -> io::Result<String> {
    let len = read_u32(reader);

    let mut buffer = Vec::new();
    reader.take(len as u64).read_to_end(&mut buffer)?;

    match str::from_utf8(&buffer) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}

pub fn read_token_or_string(reader: &mut BufReader<File>) -> io::Result<String> {
    let len = read_u32(reader);
    if len != 0 {
        read_unicode_string_with_len(reader, len)
    } else {
        read_four_byte_string(reader)
    }
}


fn read_unicode_string_with_len(reader: &mut BufReader<File>, len: u32) -> io::Result<String> {
    let mut buffer = Vec::new();
    reader.take(2 * len as u64).read_to_end(&mut buffer)?;

    let _ = read_u16(reader);
 
    match str::from_utf8(&buffer) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}

pub fn read_unicode_string(reader: &mut BufReader<File>) -> io::Result<String> {
    let len = read_u32(reader) - 1;
    read_unicode_string_with_len(reader, len)
}