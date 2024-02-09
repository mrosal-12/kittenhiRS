use serde;
use serde_json;
use egui;
use crate::messages;

type GuiVisOutput = messages::Output<egui::Context>;

pub fn gui_vis_map(cmd: String) -> messages::Command<egui::Context> {
    Box::new(|input: serde_json::Value, ctx:: &egui::Context| run(input, ctx))
}

fn run(input: serde_json::Value, ctx: &egui::Context) -> GuiVisOutput {
    //placeholder
    (None, messages::Message::None)
}