use serde;
use serde_json;
use crates::messages;

type Stateless = messages::Output<Option<bool>>;

pub fn dt_manip_map(cmd: String) -> messages::Command<Option<bool>> {
    let command_pointer = match cmd.trim() {
        "calc" => |args, _stateless| calc(args),
        "verif" => |args, _stateless| verif(args),
        _ => |_no_args, _like_its_empty| (None, messages::Message::None),
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