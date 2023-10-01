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

#[derive(Serialize, Deserialize)]
struct ActionEvent {
    expanded: bool,
    enabled: bool,
    with_dialog: bool,
    dialog_options: u8
}

fn read_action_event(reader: &mut BufReader<File>) -> ActionEvent {
    ActionEvent{
        expanded: read_bool(reader),
        enabled: read_bool(reader),
        with_dialog: read_bool(reader),
        dialog_options: read_u8(reader)
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
    let name = read_unicode_string_from_reader(reader).unwrap();
    let s1 = WStr::from_utf16be(name.as_bytes()).unwrap().to_utf8();
    let expanded = read_bool(reader);
    let nr_of_children = read_u32(reader);

    Action{
        index,
        shift_key,
        command_key,
        color_index,
        name: s1,
        expanded,
        action_events: vec![read_action_event(reader)]
    }
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

    let name = read_unicode_string_from_reader(&mut reader).unwrap();

    let s1 = WStr::from_utf16be(name.as_bytes()).unwrap().to_utf8();

    let expanded = read_bool(&mut reader);

    let nr_of_actions = read_u32(&mut reader);

    let mut actions = vec![];
    for _ in 0..nr_of_actions {
        actions.push(read_action(&mut reader));
    }

    let action_file = ActionFile{
        version,
        name: s1,
        expanded,
        actions,
    };

    println!("{}", serde_json::to_string(&action_file).unwrap());

    Ok(())
}