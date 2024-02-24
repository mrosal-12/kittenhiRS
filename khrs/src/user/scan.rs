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
use std::{thread, time::Duration};

pub fn scan() -> Vec<String> {
    let mut cam = Camera::new(
        CameraIndex::Index(0),
        RequestedFormat::new::<RgbAFormat>(RequestedFormatType::AbsoluteHighestFrameRate)
    ).unwrap();

    'res_loop: loop {

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

    }
}