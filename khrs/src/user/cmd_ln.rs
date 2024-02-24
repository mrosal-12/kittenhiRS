use std::{error::Error, io};

use crate::universals::{Signature, Entry};

pub enum UserCommand {
    Scan,
    Push(Entry),
    Pull(Signature),
    Delete(Signature),

    TeamView(usize),
    MatchView(usize),
    EventView(String),
    CloseView(usize),
    Plot(Signature, String, String),
    PieChart(Signature, String, String),
    BarChart(Signature, String, String),

    Save,
    Exit,
}

impl UserCommand {
    fn parse(input: String) -> Result<UserCommand, Box<dyn Error>> {
        let inp = input;
        //push until whitespace closure
        let puw = |sect: &mut String, pointer| { 
            let mut p = pointer;
            loop {
                if p >= inp.len() {break;}
                match inp.chars().nth(pointer).unwrap() {
                    ' ' => break,
                    x => (*sect).push(x)
                }; 
                p += 1;
            }; p
        };

        let (mut cmd, mut args1, mut args2) = (String::new(), String::new(), String::new());
        let index = puw(&mut cmd, 0);

        match cmd.as_str() {
            "scan" => Ok(UserCommand::Scan),
            "push" => {
                let entry = Entry::from(inp)?;
                Ok(UserCommand::Push(entry))
            },
            "pull" => {
                let sig = Signature::from(inp)?;
                Ok(UserCommand::Pull(sig))
            },
            "delete" => {
                let sig = Signature::from(inp)?;
                Ok(UserCommand::Delete(sig))
            },

            "team_view" => {
                let num = inp.parse::<usize>()?;
                Ok(UserCommand::TeamView(num))
            },
            "match_view" => {
                let num = inp.parse::<usize>()?;
                Ok(UserCommand::MatchView(num))
            },
            "event_view" => Ok(UserCommand::EventView(inp)),
            "close_view" => {
                let num = inp.parse::<usize>()?;
                Ok(UserCommand::CloseView(num))
            },

            "plot" => {
                let (num, sig) = Signature::drain(&inp, index)?;
                puw(&mut args2, puw(&mut args1, num));
                Ok(UserCommand::Plot(sig, args1, args2))
            },
            "pie" => {
                let (num, sig) = Signature::drain(&inp, index)?;
                puw(&mut args2, puw(&mut args1, num));
                Ok(UserCommand::PieChart(sig, args1, args2))
            },
            "bar" => {
                let (num, sig) = Signature::drain(&inp, index)?;
                puw(&mut args2, puw(&mut args1, num));
                Ok(UserCommand::BarChart(sig, args1, args2))
            },

            "exit" => Ok(UserCommand::Exit),
            "save" => Ok(UserCommand::Save),

            other => {
                println!("{} is not a valid command", other.to_string()); 
                //just to get an error that is valid, the bottom should never fire
                "a".to_string().parse::<i32>()?;
                Ok(UserCommand::Save)
            }
        }
    }
}