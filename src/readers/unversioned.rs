use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::{self, Read, BufReader};
use std::str;

use crate::readers::helpers::{read_u16, MySerializable};

pub fn read_string(reader: &mut BufReader<File>) -> io::Result<String> {
    let len = read_u16(reader);

    let mut buffer = Vec::new();
    reader.take(len as u64).read_to_end(&mut buffer)?;

    let byte_slice: &[u8] = &buffer;

    let result_string = String::from_utf8_lossy(byte_slice);

    Ok(String::from(result_string))
}

#[derive(Serialize, Deserialize)]
struct UnversionedActionFile{
    name: String,
}

pub fn read_unversioned_action_file(reader: &mut BufReader<File>) -> Box<dyn MySerializable>  {
    reader.seek_relative(-4).unwrap();

    let name = read_string(reader).unwrap();

    Box::new(UnversionedActionFile{
        name,
    })
}