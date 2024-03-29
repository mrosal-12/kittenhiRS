use std::fs;
use std::io;
use std::fs::File;
use std::path::Path;

pub fn bootup(command: Option<&String>, filename: Option<&String>) -> Option<String> {
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