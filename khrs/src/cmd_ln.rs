/*
cmd_ln acts as the main io for this.
*/
use crate::messages;
use std::io;

pub enum Mode {
    Admin,
    Editor,
    View,
}

type CmdLnOutput = messages::Output<Mode>;

pub fn cmd_ln_map(command: String) -> messages::Command<Mode> {
    let command_pointer = match command.trim() {
        "switch_mode" => |target_mode: serde_json::Value, _no_args: &Mode| switch_mode(target_mode),
        "view_mode" => |_no_args: serde_json::Value, current_mode: &Mode| view_mode(current_mode),
        "cmd_list" => |_no_args: serde_json::Value, _like_its_empty: &Mode| cmd_list(),
        "cur_cmd_list" => |_no_args: serde_json::Value, current_mode: &Mode| cur_cmd_list(current_mode),
        "scan" => |_no_args: serde_json::Value, current_mode: &Mode| scan(current_mode),
        "edit_entry" => |args: serde_json::Value, current_mode: &Mode| edit_entry(args, current_mode),
        "add_entry" => |args: serde_json::Value, current_mode: &Mode| add_entry(args, current_mode),
        "remove_entry" => |args: serde_json::Value, current_mode: &Mode| remove_entry(args, current_mode),
        "change_validity" => |args: serde_json::Value, current_mode: &Mode| change_validity(args, current_mode),
        "view_warnings" => |_no_args: serde_json::Value, current_mode: &Mode| view_warnings(current_mode),
        "close_khrs" => |_no_args: serde_json::Value, current_mode: &Mode| close_khrs(current_mode),
        "save_data" => |_no_args: serde_json::Value, current_mode: &Mode| save_data(current_mode),
        _ => |_no_args: serde_json::Value, _like_its_empty: &Mode| (None, messages::Message::None),
    };
    Box::new(command_pointer)
}

fn switch_mode(target_mode: serde_json::Value) -> CmdLnOutput { 
    if let serde_json::Value::String(some_str) = target_mode {
        match some_str.trim() {
            "view" | "View" | "viewer" | "Viewer" | "v" | "V" => (Some(Mode::View), messages::Message::None),
            "admin" | "Admin" | "adm" | "Adm" | "a" | "A" => (Some(Mode::Admin), messages::Message::None),
            "edit" | "Edit" | "editor" | "Editor" | "e" | "E" => (Some(Mode::Editor), messages::Message::None),
            _ => {println!("Invalid Argument"); (None, messages::Message::None)},
        }
    } else {
        {println!("Invalid Argument"); (None, messages::Message::None)}
    }
}

fn view_mode(current_mode: &Mode) -> CmdLnOutput { 
    println!("{}", match current_mode {
        Mode::Admin => "Admin",
        Mode::Editor => "Editor",
        Mode::View => "Viewer",
    });
    (None, messages::Message::None)
}

fn cmd_list() -> CmdLnOutput {
    println!("placeholder");
    (None, messages::Message::None)
}

fn cur_cmd_list(mode: &Mode) -> CmdLnOutput {
    //placeholder
    println!("{}", match current_mode {
        Mode::Admin => "Admin",
        Mode::Editor => "Editor",
        Mode::View => "Viewer",
    });
    (None, messages::Message::None)
}

fn scan(current_mode: &Mode) -> CmdLnOutput {
    if let Mode::Editor = current_mode {
        //placeholder until scan
        (None, messages::Message::None)
    } else {
        println!("Can only scan in Editor Mode");
        (None, messages::Message::None)
    }
}

fn edit_entry(args: serde_json::Value, current_mode: &Mode) -> CmdLnOutput {
    if let Mode::Editor = current_mode {
        (None, messages::Message::Command(
            "db_drive",
            serde_json::Value::Array(vec![
                serde_json::Value::String("change".to_string()),
                args
            ])
        ))
    } else {
        println!("Can only edit in Editor Mode");
        (None, messages::Message::None)
    }
}

fn add_entry(args: serde_json::Value, current_mode: &Mode) -> CmdLnOutput {
    if let Mode::Editor = current_mode {
        (None, messages::Message::Command(
            "db_drive",
            serde_json::Value::Array(vec![
                serde_json::Value::String("enter".to_string()),
                args
            ])
        ))
    } else {
        println!("Can only add in Editor Mode");
        (None, messages::Message::None)
    }
}

fn remove_entry(args: serde_json::Value, current_mode: &Mode) -> CmdLnOutput {
    if let Mode::Editor = current_mode {
        (None, messages::Message::Command(
            "db_drive",
            serde_json::Value::Array(vec![
                serde_json::Value::String("remove".to_string()),
                args
            ])
        ))
    } else {
        println!("Can only remove in Editor Mode");
        (None, messages::Message::None)
    }
}

fn change_validity(args: serde_json::Value, current_mode:&Mode) -> CmdLnOutput {
    if let Mode::Editor = current_mode {
        (None, messages::Message::Command(
            "db_drive",
            serde_json::Value::Array(vec![
                serde_json::Value::String("change_validity".to_string()),
                args
            ])
        ))
    } else {
        println!("Can only change validity in Editor Mode");
        (None, messages::Message::None)
    }
}

fn view_warnings(current_mode: &Mode) -> CmdLnOutput {
    if let Mode::Editor = current_mode {
        (None, messages::Message::Command(
            "db_drive".to_string(),
            serde_json::json!(["check_validity", null])
        ))
    } else {
        println!("Can only view warnings in View Mode");
        (None, messages::Message::None)
    }
}

fn close_khrs(current_mode: &Mode) -> CmdLnOutput {
    if let Mode::Editor = current_mode {
        //placeholder until scan
        (None, messages::Message::None)
    } else {
        println!("Can only close in Admin Mode");
        (None, messages::Message::None)
    }
}

fn save_data(current_mode: &Mode) -> CmdLnOutput {
    if let Mode::Editor = current_mode {
        //placeholder until scan
        (None, messages::Message::None)
    } else {
        println!("Can only save in Admin Mode");
        (None, messages::Message::None)
    }
}

pub fn simple_io(p: &str) -> (String, String) {
    println!("{}", p);            //ask what to do
    let mut input = String::new();
    if let Result::Err(err) = io::stdin().read_line(&mut input) {
        (String::new(), String::new())
    } else {
        split_input_command(input)
    }
}

fn split_input_command(input: String) -> (String, String) {
    let mut command = String::new();
    let mut argument = String::new();
    let mut is_spaced = false;
    for input_char in input.chars() {
        if is_spaced {
            command.push(input_char);
            is_spaced = input_char == ' ';
        } else {
            argument.push(input_char);
        }
    }
    (command, argument)
}