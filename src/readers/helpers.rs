use std::fs::File;
use std::io::{self, Read, BufReader};
use std::str;

use serde_json::Value;
use serde_yaml::Value as YamlValue;
use serde::{Deserialize, Serialize};

use byteorder::{BigEndian, ReadBytesExt};

pub fn read_u8(reader: &mut dyn Read) -> u8 {
    reader.read_u8().unwrap()
}

pub fn read_bool(reader: &mut dyn Read) -> bool {
    match read_u8(reader) {
        0 => false,
        1 => true,
        _ => panic!("Oh no!")
    }
}

pub fn read_u16(reader: &mut dyn Read) -> u16 {
    reader.read_u16::<BigEndian>().unwrap()
}

pub fn read_u32(reader: &mut dyn Read) -> u32 {
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


pub trait MySerializable {
    fn to_json(&self) -> Value;
    fn to_yaml(&self) -> YamlValue;
}

impl<T> MySerializable for T where T: Serialize + Deserialize<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(self).expect("JSON serialization failed")
    }

    fn to_yaml(&self) -> YamlValue {
        serde_yaml::to_value(self).expect("YAML serialization failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_bool() {
        let mut reader = &[0u8] as &[u8];
        assert_eq!(read_bool(&mut reader), false);

        let mut reader = &[1u8] as &[u8];
        assert_eq!(read_bool(&mut reader), true);
    }

    #[test]
    #[should_panic(expected = "Oh no!")]
    fn test_read_invalid_bool() {
        let mut reader = &[42u8] as &[u8];
        read_bool(&mut reader);
    }

    #[test]
    fn test_read_u8() {
        let mut reader = &[42u8] as &[u8];
        assert_eq!(read_u8(&mut reader), 42);
    }

    #[test]
    fn test_read_u16() {
        let mut reader = &[0u8, 42u8] as &[u8];
        assert_eq!(read_u16(&mut reader), 42);
    }

    #[test]
    fn test_read_u32() {
        let mut reader = &[0u8, 0u8, 0u8, 42u8] as &[u8];
        assert_eq!(read_u32(&mut reader), 42);
    }
}