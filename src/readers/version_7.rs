use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;
use std::str;

use crate::readers::helpers::{read_u32, read_bool, read_u8, read_four_byte_string, read_string, read_token_or_string, MySerializable};


#[derive(Serialize, Deserialize)]
struct V7ActionFile{
    version: u32,
    field_1: u32,
    field_2: u32,
    name: String,
    // expanded: bool,
    // actions: Vec<Action>
}

pub fn read_version_7_action_file(reader: &mut BufReader<File>) -> Box<dyn MySerializable>  {
    let field_1 = read_u32(reader);
    let field_2 = read_u32(reader);
    let name = read_string(reader).unwrap();

    Box::new(V7ActionFile{
        version: 7,
        field_1,
        field_2,
        name,
    })
}

