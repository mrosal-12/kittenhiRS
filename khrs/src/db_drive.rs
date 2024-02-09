/*
db_drive acts as the link between rust and storage
*/
use std::fs;
use serde;
use serde_json;
use crate::messages;

type DbDriveOutput = messages::Output<String>;

pub fn db_drive_map(cmd: String) -> messages::Command<String> {
    let command_pointer = match cmd.trim() {
        "read" => |ent: serde_json::Value, fp: &String| read(ent, fp),
        "read_team" => |ent: serde_json::Value, fp: &String| read_team(ent, fp),
        "read_match" => |ent: serde_json::Value, fp: &String| read_match(ent, fp),
        "read_event" => |ent: serde_json::Value, fp: &String| read_event(ent, fp),
        "enter" => |ent: serde_json::Value, fp: &String| enter(ent, fp),
        "remove" => |ent: serde_json::Value, fp: &String| remove(ent, fp),
        "change" => |ent: serde_json::Value, fp: &String| change(ent, fp),
        "check_validity" => |_ent: serde_json::Value, fp: &String| check_validity(fp),
        "change_validity" => |ent: serde_json::Value, fp: &String| change_validity(ent, fp),
        _ => |_no_args: serde_json::Value, _like_its_empty: &String| (None, messages::Message::None),
    };
    Box::new(command_pointer)
}

/*
Takes in a Destination (String) and a Target Entry (Array) 
Returns a destination and the entry
*/
fn read(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // placeholder
    (None, messages::Message::None)
}

/*
Takes in a Destination (String) and a Target Team (String) 
Returns a destination and the Teams
*/
fn read_team(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // placeholder
    (None, messages::Message::None)
}

/*
Takes in a Destination (String) and 6 Target Teams (Array) 
Returns a destination and the current & average stats for Teams
*/
fn read_match(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // placeholder
    (None, messages::Message::None)
}

/*
Takes in a Destination (String) 
Returns a destination and the current & average stats for all teams
*/
fn read_event(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // placeholder
    (None, messages::Message::None)
}

/*
Takes in a Input Array (Array) 
Converts and Writes an Output Array to Storage 
*/
fn enter(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // put logic here
    (None, messages::Message::None)
}

/*
Takes in an Entry
Remove the Entry
*/
fn remove(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // put logic here
    (None, messages::Message::None)
}

/*
Takes in a Destination and an Output
Checks if the output is the same form as destination, if it is, change it
*/
fn change(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // put logic here
    (None, messages::Message::None)
}

fn check_validity(file_path: &String) -> DbDriveOutput {
    // placeholder
    (None, messages::Message::None)
}

fn change_validity(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // placeholder
    (None, messages::Message::None)
}