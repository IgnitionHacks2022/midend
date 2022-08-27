#![allow(unused)]
#![allow(dead_code)]

mod api;
mod bluetooth;
mod camera;
mod models;
mod motion;
mod pi_gpio;

use std::{sync::mpsc, thread};

use anyhow::Result;
use models::Item;
use pino_utils::ok_or_continue_msg;

use crate::camera::take_picture;

fn main() {
    // let image = take_picture("/dev/video2").unwrap();
    // println!("{:?}", image);
    // classify("nithin", image).unwrap();
    // gpio_test().unwrap();
    let (motion_tx, motion_rx) = mpsc::channel::<Vec<u8>>();
    let (gpio_tx, gpio_rx) = mpsc::channel::<Item>();

    let opencv_handle = thread::spawn(move || {
        motion::opencv_test(motion_tx, 2).unwrap();
    });
    let api_handle = thread::spawn(move || {
        for recv in motion_rx {
            println!("recieved");
            let resp = ok_or_continue_msg!(api::classify("nithin", recv), |e| {
                // println!("{:?}", e);
            });
            if gpio_tx.send(resp.item_type).is_err() {
                println!("Error sending to gpio thread");
            }
        }
    });
    let gpio_handle = thread::spawn(move || {
        for recv in gpio_rx {
            println!("motor for item {:?}", recv);
        }
    });

    opencv_handle.join().unwrap();
}
