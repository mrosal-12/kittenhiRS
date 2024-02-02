use std::env;
use std::fs;
use std::io;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::sync::mpsc;
use std::time;
use messages::Message;
use serde;
use serde_json;

mod cmd_ln; //command line portion of the app
mod db_drive; //thread that communicates with the linked database
mod dt_manip; //thread that does backend calculations and data verification
mod gui_vis; //thread for viewing data visually
mod handeling;
mod messages;
mod bootup_and_closing;

fn main() {
    // Bootup and Loading 
    let args: Vec<String> = env::args().collect();

    // Load file path with the two arguments
    let file_path: String = 
        if args.len() == 2 {bootup_and_closing::bootup(args.get(0), args.get(1)).unwrap()}
        else {panic!("Must load program with 2 arguments via the command line")};

    // load the threads
    //do the channels first
        // These transmitted from the main thread
    let (db_drive_tx, db_drive_rx) = mpsc::channel::<messages::Message>();
    let (dt_manip_tx, dt_manip_rx) = mpsc::channel::<messages::Message>();
    let (gui_vis_tx, gui_vis_rx) = mpsc::channel::<messages::Message>();
        // These are recieved on the main thread
    let (cmd_ln_c, rx) = mpsc::channel::<messages::Message>();
    let db_drive_c = cmd_ln_c.clone();
    let dt_manip_c = cmd_ln_c.clone();
    let gui_vis_c = cmd_ln_c.clone();

    // load in the threads now //Note that rn main is a placeholder
    let cmd_ln_thread = thread::spawn(move || {     //spawn cmd_ln
        let mut mode = cmd_ln::Mode::View;       //cmd_lns behavior changes as mode changes, couldn't find a way to do this immutably w/out recursion
        loop {                                  //cmd_ln event loop
            let (cmd_ln_state, cmd_ln_out) = {          //these are what the handeler outputs
                let (cmd_in1, cmd_in2) = cmd_ln::simple_io("Input Available");
                messages::Message::from(cmd_in1.trim(), cmd_in2.trim())
                    .find_handle(Box::new(|cmd| cmd_ln::cmd_ln_map(cmd)))
                    .run(&mode)
            };

            //change mode if it wants too
            if let Some(state) = cmd_ln_state {
                mode = state;
            }

            //keep trying to send to main
            while let Err(_) = cmd_ln_c.send(cmd_ln_out.clone()) {
                println!("cmd_ln had error sending");
                thread::sleep(time::Duration::from_secs(5));
            }
        }
    });

    let db_drive_thread = thread::spawn(move || {
        for recieved in db_drive_rx {
            let (_, db_drive_out) = {
                recieved
                    .find_handle(Box::new(|cmd| db_drive::db_drive_map(cmd)))
                    .run(&file_path)
            };

            while let Err(_) = db_drive_c.send(db_drive_out.clone()) {
                println!("db_drive had error sending");
                thread::sleep(time::Duration::from_secs(5));
            }
        }
    });

    let dt_manip_thread = thread::spawn(move || {
        let _dtm_env = Option::None;
        for recieved in dt_manip_rx {
            let (_, dt_manip_out) = {
                recieved
                    .find_handle(Box::new(|cmd| dt_manip::dt_manip_map(cmd)))
                    .run(&_dtm_env)
            };

            while let Err(_) = dt_manip_c.send(dt_manip_out.clone()) {
                println!("dt_manip had error sending");
                thread::sleep(time::Duration::from_secs(5));
            }
        }
    });

    /* 
    let gui_vis_thread = thread::spawn(move || {
        let _gv_env = Option::None;
        for recieved in gui_vis_rx {
            let (_, gui_vis_out) = {
                recieved
                    .find_handle(Box::new(|cmd| gui_vis::gui_vis_map(cmd)))
                    .run(&_gv_env)
            };

            while let Err(_) = gui_vis_c.send(gui_vis_out) {
                println!("gui_vis had error sending");
                thread::sleep(time::Duration::from_secs(5));
            }
        }
    }); */

    let mut target = handeling::Destination::None; //tells the target for message sending
    // load in the event loop
    for recieved in rx {
        let (main_state, main_out) = {              //get output
            recieved
                .find_handle(Box::new(|_cmd| handeling::handler()))
                .run(&target)
        };

        //change target if nec.
        if let Some(state) = main_state {
            target = state;
        }

        //keep sending message to target
        match target {
            handeling::Destination::None => (),

            handeling::Destination::DbDrive => while let Err(_) = db_drive_tx.send(main_out.clone()) {
                println!("cmd_ln had error sending");
                thread::sleep(time::Duration::from_secs(5));
            },

            handeling::Destination::DtManip => while let Err(_) = dt_manip_tx.send(main_out.clone()) {
                println!("cmd_ln had error sending");
                thread::sleep(time::Duration::from_secs(5));
            },

            handeling::Destination::GuiVis => while let Err(_) = gui_vis_tx.send(main_out.clone()) {
                println!("cmd_ln had error sending");
                thread::sleep(time::Duration::from_secs(5));
            },
        }
    }

}