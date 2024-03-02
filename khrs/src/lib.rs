use bardecoder;
use nokhwa::{
    Camera,
    utils::{
        CameraIndex, 
        RequestedFormat, 
        RequestedFormatType
    },
    pixel_format::RgbAFormat
};
use std::{
    thread, 
    time::Duration,
    fs,
    io
};

pub fn scan() -> Vec<String> {
    let mut cam = Camera::new(
        CameraIndex::Index(0),
        RequestedFormat::new::<RgbAFormat>(RequestedFormatType::AbsoluteHighestFrameRate)
    ).unwrap();

    let a = &'res_loop: loop {

        let scan = loop {
            if let Ok(buf) = cam.frame() {
                if let Ok(dec) = buf.decode_image::<RgbAFormat>() {
                    break dec;
                }
            } thread::sleep(Duration::from_secs(5));
        };

        let mut scn = Vec::new();
        for thing in bardecoder::default_decoder().decode(&scan) { 
            if let Ok(el) = thing {scn.push(el);}
            else {thread::sleep(Duration::from_secs(5)); continue 'res_loop;}
        }
        if scn.len() != 0 {break scn;}

    }[0];

    string_to_vec(a.to_string())
}
 
pub fn push(str: Vec<String>, db: &mut Vec<Vec<String>>) {
    (*db).push(str);
}

pub fn find_team(team: &String, db: &Vec<Vec<String>>) -> Vec<Vec<String>> { 
    <Vec<Vec<std::string::String>> as Clone>::clone(&(*db)).into_iter()
        .filter(|vec| vec[5].as_str() == team.as_str())
        .collect()
}

pub fn find_evm(event: &String, mat: &String, db: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    <Vec<Vec<std::string::String>> as Clone>::clone(&(*db)).into_iter()
        .filter(|vec| vec[1].as_str() == event.as_str() && vec[3].as_str() == mat.as_str())
        .collect()
}

pub fn save(db: Vec<Vec<String>>) -> io::Result<()> {
    let mut str = String::new();
    for vec in db {
        for s in vec {
            str.push_str((s + " ").as_str());
        } str.push_str("\n");
    } fs::write("storage.txt", str)
}

pub fn string_to_vec(s: String) -> Vec<String> {
    let mut len = 0;
    let mut vec = Vec::new();
    vec.push(String::new());
    for c in s.chars() {
        if c == ' ' {vec.push(String::new()); len += 1;}
        else {vec[len].push(c);}
    }
    vec
}