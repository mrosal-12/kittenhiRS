/*
db_drive acts as the link between rust and storage
*/
use std::fs;
use serde;
use serde_json;
use crate::messages;

type DbDriveOutput = messages::Output<String>

pub fn db_drive_map(cmd: String) -> messages::Command<String> {
    let command_pointer = match cmd.trim() {
        "read" => |ent, fp| read(ent, fp),
        "enter" => |ent, fp| enter(ent, fp),
        "remove" => |ent, fp| remove(ent, fp),
        "change" => |ent, fp| change(ent, fp),
        "check_validity" => |_ent, fp| check_validity(fp),
        "change_validity" => |ent, fp| change_validity(ent, fp),
        _ => |_no_args, _like_its_empty| (None, messages::Message::None),
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

fn check_validity(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // placeholder
    (None, messages::Message::None)
}

fn change_validity(entry: serde_json::Value, file_path: &String) -> DbDriveOutput {
    // placeholder
    (None, messages::Message::None)
}