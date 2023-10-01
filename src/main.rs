use clap::Parser;

use std::fs::File;
use std::io::{self, Read, BufReader};
use std::str;

use std::path::PathBuf;

use byteorder::{BigEndian, ReadBytesExt};

use utf16string::{WStr};

fn read_u8(reader: &mut BufReader<File>) -> u8 {
    reader.read_u8().unwrap()
}

fn read_u16(reader: &mut BufReader<File>) -> u16 {
    reader.read_u16::<BigEndian>().unwrap()
}

fn read_u32(reader: &mut BufReader<File>) -> u32 {
    reader.read_u32::<BigEndian>().unwrap()
}


fn read_version(reader: &mut BufReader<File>) -> u32 {
    read_u32(reader)
}

fn read_unicode_string_from_reader(reader: &mut BufReader<File>) -> io::Result<String> {
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

fn dump_bool_field(key: &str, value: u8) {
    let b =  match value {
        0 => false,
        1 => true,
        _ => panic!("Oh no!")
    };
    println!("\"{}\": {},", key, b);
}


fn dump_u8_field(key: &str, value: u8) {
    println!("\"{}\": {},", key, value);
}

fn dump_u16_field(key: &str, value: u16) {
    println!("\"{}\": {},", key, value);
}

fn dump_u32_field(key: &str, value: u32) {
    println!("\"{}\": {},", key, value);
}

fn open_json() {
    println!("{{");
}

fn close_json() {
    println!("}}");
}

fn read_action_event(reader: &mut BufReader<File>) {
    let expanded = read_u8(reader);
    let enabled = read_u8(reader);
    let with_dialog = read_u8(reader);
    let dialog_options =read_u8(reader);

    open_json();
    dump_bool_field("expanded", expanded);
    dump_bool_field("enabled", enabled);
    dump_u8_field("with_dialog", with_dialog);
    println!("\"{}\": {}", "dialog_optons", dialog_options);
    close_json();
}

fn read_action(reader: &mut BufReader<File>) {
    let index = read_u16(reader);
    let shift_key = read_u8(reader);
    let command_key = read_u8(reader);
    let color_index = read_u16(reader);
    let name = read_unicode_string_from_reader(reader).unwrap();
    let s1 = WStr::from_utf16be(name.as_bytes()).unwrap().to_utf8();
    let expanded = read_u8(reader);
    let nr_of_children = read_u32(reader);


    open_json();
    dump_u16_field("index", index);
    dump_u8_field("shift_key", shift_key);
    dump_u8_field("command_key", command_key);
    dump_u16_field("color_index", color_index);
    println!("\"name\": \"{}\",", s1);
    dump_bool_field("expanded", expanded);
    dump_u32_field("nor_of_children", nr_of_children);
    println!("\"action\": [");
    // for _ in 0..nr_of_children-1 {
        read_action_event(reader);
        // println!(",");
    // }
    // read_action_event(reader);
    println!("]");
    close_json();
}


#[derive(Parser)]
struct Cli {
    path: Option<PathBuf>
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let file_name = args.path.unwrap_or(PathBuf::from("test.atn"));

    let file = File::open(file_name)?;

    let mut reader = BufReader::new(file);

    let version = read_version(&mut reader);

    let name = read_unicode_string_from_reader(&mut reader).unwrap();

    let s1 = WStr::from_utf16be(name.as_bytes()).unwrap().to_utf8();

    let expanded = read_u8(&mut reader);

    let nr_of_actions = read_u32(&mut reader);

    println!("{{");
    println!("\"version\": {},", version);
    println!("\"name\": \"{}\",", s1);
    dump_bool_field("expanded", expanded);
    println!("\"nr_of_actions\": {},", nr_of_actions,);
    println!("\"action\": [");
    for _ in 0..nr_of_actions {
        read_action(&mut reader)
    }
    println!("]");
    println!("}}");

    Ok(())
}