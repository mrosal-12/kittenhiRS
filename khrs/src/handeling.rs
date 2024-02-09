use serde_json;
use crate::messages;

pub enum Destination {
    None,
    DbDrive,
    DtManip,
    GuiVis,
}

pub fn handler(cmd: String) -> messages::Command<Destination> {
    Box::new(|args, _env| {
        (
            match cmd.trim() {
                "db_drive" => Some(Destination::DbDrive),
                "dt_manip" => Some(Destination::DbDrive),
                "gui_vis" => Some(Destination::DbDrive),
                _ => Some(Destination::None),
            },

            messages::Message::Command(
                match &args[0] {
                    serde_json::Value::String(cmd) => cmd.to_string(),
                    _ => "".to_string(),
                },

                match &args[1] {
                    serde_json::Value::Bool(b) => serde_json::Value::Bool(*b),
                    serde_json::Value::Number(b) => serde_json::Value::Number(b.clone()),
                    serde_json::Value::String(b) => serde_json::Value::String(b.clone()),
                    serde_json::Value::Array(b) => serde_json::Value::Array(b.clone()),
                    serde_json::Value::Object(b) => serde_json::Value::Object(b.clone()),
                    _ => serde_json::Value::Null,
                }
            )
        )
    })
}

