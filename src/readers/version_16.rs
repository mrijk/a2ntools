use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;
use std::str;

use utf16string::WStr;

use crate::readers::helpers::{read_unicode_string, read_u16, read_u32, read_bool, read_u8, read_four_byte_string, read_string, read_token_or_string};


fn read_name(reader: &mut BufReader<File>) -> String {
    let name = read_unicode_string(reader).unwrap();
    WStr::from_utf16be(name.as_bytes()).unwrap().to_utf8()
}


#[derive(Serialize, Deserialize)]
struct Item {
    key: String
}

#[derive(Serialize, Deserialize)]
struct StringStructure {
    value: String,
    foo: u32,
}

#[derive(Serialize, Deserialize)]
enum Foo {
    Item(Item),
    StringStructure(StringStructure,)
}


#[derive(Serialize, Deserialize)]
struct Descriptor {
    class_id_1: String,
    class_id_2: String,
    nr_of_items: u32,
    items: Vec<Foo>,
}

#[derive(Serialize, Deserialize)]
struct ActionEvent {
    expanded: bool,
    enabled: bool,
    with_dialog: bool,
    dialog_options: u8,
    event_name: String,
    dictionary: String,
    descriptor: Option<Descriptor>,
}

fn read_event_name(reader: &mut BufReader<File>) -> String {
    let event_name = match read_four_byte_string(reader) {
        Ok(s) => match s.as_str() {
            "TEXT" => read_string(reader).unwrap(),
            "long" => read_four_byte_string(reader).unwrap(),
            _ => panic!("disaster")
        },
        Err(_e) => panic!("diaaster")
    };
    event_name
}

fn read_item(reader: &mut BufReader<File>) -> Foo {
    let key = read_token_or_string(reader).unwrap();
    Foo::Item(Item{key})
}

fn read_descriptor(reader: &mut BufReader<File>) -> Descriptor {
    let class_id_1 = read_unicode_string(reader).unwrap();
    let class_id_2 = read_token_or_string(reader).unwrap();
    let nr_of_items = read_u32(reader);
    let foo = StringStructure{value: String::new(), foo: 666};
    let test = Foo::StringStructure(foo);
    let items = vec![read_item(reader), test];

    Descriptor {
        class_id_1,
        class_id_2,
        nr_of_items,
        items,
    }
}

fn read_action_event(reader: &mut BufReader<File>) -> ActionEvent {
    let expanded = read_bool(reader);
    let enabled = read_bool(reader);
    let with_dialog = read_bool(reader);
    let dialog_options = read_u8(reader);

    let event_name = read_event_name(reader);

    let dictionary = read_string(reader).unwrap();

    let has_descriptor = read_u32(reader) != 0;
    let descriptor = match has_descriptor {
        true => Some(read_descriptor(reader)),
        false => None
    };

    ActionEvent{
        expanded,
        enabled,
        with_dialog,
        dialog_options,
        event_name,
        dictionary,
        descriptor,
    }
}

fn read_action_events(reader: &mut BufReader<File>) -> Vec<ActionEvent> {
    let _nr_of_action_events = read_u32(reader);

    (0..1).map(|_| read_action_event(reader)).collect()
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
    let action_events = read_action_events(reader);

    Action{
        index,
        shift_key,
        command_key,
        color_index,
        name,
        expanded,
        action_events
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

pub fn read_version_16_action_file(reader: &mut BufReader<File>) {
    let name = read_name(reader);
    let expanded = read_bool(reader);
    let actions = read_actions(reader);

    let action_file = ActionFile{
        version: 16,
        name,
        expanded,
        actions,
    };

    println!("{}", serde_json::to_string(&action_file).unwrap());
}