use serde::{Deserialize, Serialize};
// use serde_json::Result;

use clap::Parser;

use std::fs::File;
use std::io::{self, BufReader};
use std::str;

use std::path::PathBuf;

use utf16string::WStr;

mod helpers;
use crate::helpers::{read_unicode_string, read_u16, read_u32, read_bool, read_u8, read_four_byte_string, read_string, read_token_or_string};

fn read_version(reader: &mut BufReader<File>) -> u32 {
    read_u32(reader)
}

fn read_name(reader: &mut BufReader<File>) -> String {
    let name = read_unicode_string(reader).unwrap();
    WStr::from_utf16be(name.as_bytes()).unwrap().to_utf8()
}

#[derive(Serialize, Deserialize)]
struct ActionEvent {
    expanded: bool,
    enabled: bool,
    with_dialog: bool,
    dialog_options: u8,
    event_name: String,
    has_descriptor: bool,
}

fn read_action_event(reader: &mut BufReader<File>) -> ActionEvent {
    let expanded = read_bool(reader);
    let enabled = read_bool(reader);
    let with_dialog = read_bool(reader);
    let dialog_options = read_u8(reader);

    let event_name = match read_four_byte_string(reader) {
        Ok(s) => match s.as_str() {
            "TEXT" => read_string(reader).unwrap(),
            "long" => read_four_byte_string(reader).unwrap(),
            _ => panic!("disaster")
        },
        Err(e) => panic!("diaaster")
    };

    let tmp = read_string(reader).unwrap();

    let has_descriptor = (read_u32(reader) != 0);

    let class_id_1 = read_unicode_string(reader).unwrap();
    let class_id_2 = read_token_or_string(reader).unwrap();

    let nr_of_itmes = read_u32(reader);

    ActionEvent{
        expanded,
        enabled,
        with_dialog,
        dialog_options,
        event_name,
        has_descriptor
    }
}

#[derive(Serialize, Deserialize)]
struct Action {
    index: u16,
    shift_key: bool,
    command_key: bool,
    color_index: u16,
    name: String,
    expanded: bool,
    action_events: Vec<ActionEvent>
}

fn read_action(reader: &mut BufReader<File>) -> Action {
    let index = read_u16(reader);
    let shift_key = read_bool(reader);
    let command_key = read_bool(reader);
    let color_index = read_u16(reader);
    let name = read_name(reader);
    let expanded = read_bool(reader);
    let _nr_of_children = read_u32(reader);

    Action{
        index,
        shift_key,
        command_key,
        color_index,
        name,
        expanded,
        action_events: vec![read_action_event(reader)]
    }
}

fn read_actions(reader: &mut BufReader<File>) -> Vec<Action> {
    let nr_of_actions = read_u32(reader);

    (0..nr_of_actions).map(|_| read_action(reader)).collect()
}

#[derive(Serialize, Deserialize)]
struct ActionFile{
    version: u32,
    name: String,
    expanded: bool,
    actions: Vec<Action>
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
    let name = read_name(&mut reader);
    let expanded = read_bool(&mut reader);
    let actions = read_actions(&mut reader);

    let action_file = ActionFile{
        version,
        name,
        expanded,
        actions,
    };

    println!("{}", serde_json::to_string(&action_file).unwrap());

    Ok(())
}