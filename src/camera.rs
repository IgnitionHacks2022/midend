use anyhow::Result;
use rscam::*;
use std::{fs::File, io::Write};
use pino_utils::{ok_or_continue, ok_or_continue_msg};

pub fn take_picture(cam_device: &str) -> Result<()> {

    let err_fn = |e| { println!("{:?}", e); };

    let mut camera = rscam::new(cam_device)?;

    camera.start(&Config {
        interval: (1, 30),
        resolution: (1280, 720),
        format: b"MJPG",
        ..Default::default()
    }).unwrap();

    for i in 0..10 {
        let frame = ok_or_continue_msg!(camera.capture(), err_fn);
        let mut file = ok_or_continue_msg!(File::create(&format!("out/frame-{}", i)), err_fn);
        ok_or_continue!(file.write_all(&frame[..]));
    }

    Ok(())
}

