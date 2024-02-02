use serde;
use serde_json;
use crate::messages;

type Stateless = messages::Output<Option<bool>>;

pub fn dt_manip_map(cmd: String) -> messages::Command<Option<bool>> {
    let command_pointer = match cmd.trim() {
        "calc" => |args: serde_json::Value, _stateless: &Option<bool>| calc(args),
        "verif" => |args: serde_json::Value, _stateless: &Option<bool>| verif(args),
        _ => |_no_args: serde_json::Value, _like_its_empty: &Option<bool>| (None, messages::Message::None),
    };
    Box::new(command_pointer)
}

fn calc(args: serde_json::Value) -> Stateless {
    //placeholder
    (None, messages::Message::None)
}

fn verif(args: serde_json::Value) -> Stateless {
    //placeholder
    (None, messages::Message::None)
}