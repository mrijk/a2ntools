use serde::{Deserialize, Serialize};
use serde_json::Result;

use clap::Parser;

use std::fs::File;
use std::io::{self, BufReader};
use std::str;

use std::path::PathBuf;

use utf16string::{WStr};

mod helpers;
use crate::helpers::{read_unicode_string_from_reader, read_u16, read_u32, read_bool, read_u8};

fn read_version(reader: &mut BufReader<File>) -> u32 {
    read_u32(reader)
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

#[derive(Serialize, Deserialize)]
struct ActionEvent {
    expanded: bool,
    enabled: bool,
    with_dialog: bool,
    dialog_options: u8
}

fn read_action_event(reader: &mut BufReader<File>) {
    let expanded = read_bool(reader);
    let enabled = read_bool(reader);
    let with_dialog = read_bool(reader);
    let dialog_options =read_u8(reader);

    let action_event = ActionEvent{
        expanded: expanded,
        enabled: enabled,
        with_dialog: with_dialog,
        dialog_options: dialog_options
    };

    println!("{}", serde_json::to_string(&action_event).unwrap());
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