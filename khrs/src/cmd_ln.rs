/*
cmd_ln acts as the main io for this.
*/
mod admin; //module for all your admin needs (i.e. control entire system)
mod edit; //module for all editing stuff - manual editing and scanning in data
mod view; //module for all viewing stuff (data, validity, etc)

pub enum Mode {
    Admin,
    Editor,
    View,
}

impl Mode {
    fn switch_modes(&mut self, Mode) {
        self = Mode;
    }
}

pub fn run_command(command: &Vec<String>, &mut mode: Mode) -> Option<&str> {
    let cmd = command.get(0)?.trim();
    let v = command!;
    let mut b = false;
    let args: Vec<i32> = Vec::new();
    for i in &v {
        if b {args.push(i);}
        else {b = true;}
    }
    match mode {
        Admin => match cmd {
            "open_gui" => Admin::open_gui(args),
            "close_gui" => Admin::close_gui(args),
            "check_path" => Admin::check_path(args),
            "close_khrs" => Admin::close_khrs(args),
            "switch" => {
                let m = args.get(0)?;
                match m {
                    "view" => mode.switch_modes(Mode::View),
                    "edit" => mode.switch_modes(Mode::Editor),
                    "admin" => mode.switch_modes(Mode::Admin),
                    _ => println!("{} is not a valid command for Switch", m),
                };
                Option::None
            },
            _ => {println!("{} is not a valid command for Admin Mode", cmd); Option::None},
        },

        Editor => match cmd {
            "scan" => Editor::scan(args),
            "remove_entry" => Editor::remove_entry(args),
            "add_entry" => Editor::add_entry(args),
            "edit_entry" => Editor::edit_entry(args),
            "change_validity" => Editor::change_validity(args),
            "switch" => {
                let m = args.get(0)?;
                match m {
                    "view" => mode.switch_modes(Mode::View),
                    "edit" => mode.switch_modes(Mode::Editor),
                    "admin" => mode.switch_modes(Mode::Admin),
                    _ => println!("{} is not a valid command for Switch", m),
                };
                Option::None
            },
            _ => {println!("{} is not a valid command for View Mode", cmd); Option::None},
        },

        View => match cmd {
            "view_warnings" => View::view_warnings(args),
            "view_entry" => View::view_entry(args),
            "switch" => {
                let m = args.get(0)?;
                match m {
                    "view" => mode.switch_modes(Mode::View),
                    "edit" => mode.switch_modes(Mode::Editor),
                    "admin" => mode.switch_modes(Mode::Admin),
                    _ => println!("{} is not a valid command for Switch", m),
                };
                Option::None
            },
            _ => {println!("{} is not a valid command for View Mode", cmd); Option::None},
        },
    }
}