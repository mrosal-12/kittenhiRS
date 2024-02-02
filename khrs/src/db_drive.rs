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
        "enter" => |ent: serde_json::Value, fp: &String| enter(ent, fp),
        "remove" => |ent: serde_json::Value, fp: &String| remove(ent, fp),
        "change" => |ent: serde_json::Value, fp: &String| change(ent, fp),
        "check_validity" => |_ent: serde_json::Value, fp: &String| check_validity(fp),
        "change_validity" => |ent: serde_json::Value, fp: &String| change_validity(ent, fp),
        _ => |_no_args: serde_json::Value, _like_its_empty: &String| (None, messages::Message::None),
    };
    Box::new(command_pointer)
}

fn read(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // placeholder
    (None, messages::Message::None)
}

fn enter(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // placeholder
    (None, messages::Message::None)
}

fn remove(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // placeholder
    (None, messages::Message::None)
}

fn change(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // placeholder
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