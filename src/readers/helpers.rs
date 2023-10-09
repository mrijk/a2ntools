use std::fs::File;
use std::io::{self, Read, BufReader};
use std::str;

use serde_json::Value;
use serde_yaml::{Value as YamlValue};
use serde::{Deserialize, Serialize};

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