use std::io;

use khrs::{find_evm, find_team, push, save, scan, string_to_vec};

fn main() {
    let mut db = Vec::new();
    let mut input = String::new();
    loop {
        let mut vec = Vec::new();
        {
        io::stdin()
            .read_line(&mut input)
            .unwrap_or_else(|_|0);
        }
        let inp = string_to_vec(input.clone());
        if inp.len() < 1 {println!("No Command");}
        else {
            match inp[0].as_str() {
                "scan" => push(scan(), &mut db),
                "pull_team" => vec = find_team(&inp[1], &db),
                "pull_evmm" => vec = find_evm(&inp[1], &inp[2], &db),
                "save" => {save(db); break;}
                _ => (),
            }
            if vec.len() > 0 {
                let mut str = String::new();
                for vec in &db {
                    for s in vec {
                        str.push_str((s.to_owned() + " ").as_str());
                    } str.push_str("\n");
                } println!("{}", str.as_str())
            }
        }
    }
}