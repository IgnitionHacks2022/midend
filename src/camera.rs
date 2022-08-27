use std::{fs::File, io::Write};

use anyhow::Result;
use pino_utils::{ok_or_continue, ok_or_continue_msg};
use rscam::*;

pub fn take_picture(cam_device: &str) -> Result<Vec<u8>> {
    let mut camera = rscam::new(cam_device)?;

    camera
        .start(&Config {
            interval: (1, 30),
            resolution: (1280, 720),
            format: b"MJPG",
            ..Default::default()
        })
        .unwrap();

    let frame = camera.capture()?;
    Ok(frame[..].to_vec())
}
