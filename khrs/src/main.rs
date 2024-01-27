use std::env;
use std::fs;
use std::io;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::sync::mpsc;
use serde;
use serde_json;

mod cmd_ln; //command line portion of the app
mod db_drive; //thread that communicates with the linked database
mod dt_manip; //thread that does backend calculations and data verification
mod editor; //thread for manually editing and scanning in data
mod gui_vis; //thread for viewing data visually
mod handeling;

fn main() {
    // Bootup and Loading 
    let args: Vec<String> = env::args().collect();

    // Load file path with the two arguments
    let file_path: String = 
        if args.len() == 2 {bootup(args.get(0), args.get(1)).unwrap()}
        else {panic!("Must load program with 2 arguments via the command line")};

    // load the threads
    //do the channels first
        // These transmitted from the main thread
    let (db_drive_tx, db_drive_rx) = mpsc::channel();
    let (dt_manip_tx, dt_manip_rx) = mpsc::channel();
    let (editor_tx, editor_rx) = mpsc::channel();
    let (gui_vis_tx, gui_vis_rx) = mpsc::channel();
        // These are recieved on the main thread
    let (cmd_ln_c, rx) = mpsc::channel();
    let db_drive_c = cmd_ln_c.clone();
    let dt_manip_c = cmd_ln_c.clone();
    let editor_c = cmd_ln_c.clone();
    let gui_vis_c = cmd_ln_c.clone();

    // load in the threads now //Note that rn main is a placeholder
    let cmd_ln_thread = thread::spawn(move || cmd_ln::main());
    let db_drive_thread = thread::spawn(move || db_drive::main());
    let dt_manip_thread = thread::spawn(move || dt_manip::main());
    let editor_thread = thread::spawn(move || editor::main());
    let gui_vis_thread = thread::spawn(move || gui_vis::main());

    // load in the event loop
    loop {
        //check each reciever for a message
        //do logic stuff
        //pass on to appropriate channel
    }

}


/*
    PARAMETERS:
    cmd: &str | a string slice representing the bootup command (either "load" or "new")
    file: &str | a strign slice representing the name of the linked storage file

    returns a string slice representing the relative file path of the linked storage file
*/
fn bootup(command: Option<&String>, filename: Option<&String>) -> Option<String> {
    let cmd = command?.trim();      //convert to slice
    let file = filename?.trim();
    let temp = ["./storage/", file, ".json"].concat();
    let fp: &str = temp.trim();    //create a file path
    let exists = Path::new(fp).try_exists();        //will check for error later

    match cmd {
        "new" => {      //for the "new" command
            {           //confirmation
            println!("Are you sure wish to create {}? Type Y to confirm", file);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            if input.trim() != "Y" {panic!("Failed to load file")};
            }

            match exists {
                Ok(x) if x => {       //if the file path exists
                    println!("{} already exists. Load (L) or Overwrite (D)?", file);            //ask what to do
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    while input.trim() != "L" && input.trim() != "D" {      //if the user doesn't put in L or D
                        println!("Load(L) or Overwrite (D)?");
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                    }
                    
                    match input.trim() {
                        "L" => bootup(Some(&String::from("load")), Some(&String::from(file))),        //Load instead of new
                        "D" => {
                            println!("Type DELETE to confirm overwriting of {}", file); //please make sure this isn't a mistake
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            match input.trim() {        
                                "DELETE" => {           //Okay, if you say so
                                    match fs::remove_file(fp) {
                                        Ok(_x) => bootup(Some(&String::from("new")), Some(&String::from(file))),     //now make a new file
                                        Err(_x) => panic!("Cannot remove a file"),      //throw an error if you cant
                                    }
                                },
                                _ => panic!("Failed to load file"),     //ok if you don't wanna lets just crash the program and start over
                            }
                        },
                        _ => panic!("unexplainable error"),
                    }
                },

                Ok(x) if !x => {      //if it doesn't exist create it and return the file path
                    match File::create(fp) {
                        Ok(_x) => {println!("{} created", fp); Some(String::from(fp))},
                        Err(_x) => panic!("Failed to create file"),             //throw an error if you can't
                    }
                },

                _ => panic!("unexplainable error"),     //throw an error if otherwise
            }
        },

        "load" => {     //load target file
            match exists {
                Ok(x) if x => {       //if it exists, return file path
                    println!("Successfully Loaded {}", file);
                    Some(String::from(fp))
                },

                Ok(x) if !x => {      //if it doesn't exist, ask if they wanted to create a new one
                    {
                    println!("couldn't find {}. Create new file (Y to confirm)?", file);
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    if input.trim() != "Y" {panic!("Failed to load file")};
                    }

                    bootup(Some(&String::from("new")), Some(&String::from(file)))     //create a new one
                },

                _ => panic!("unexplainable error"),     //throw an error if otherwise
            }
        },

        _ => panic!("Command has to be load or new. Remeber that case matters!"),       //not the correct command lol
    }
}