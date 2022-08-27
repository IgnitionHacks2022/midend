#![allow(unused)]
#![allow(dead_code)]

mod api;
mod audio;
mod bluetooth;
mod camera;
mod models;
mod motion;
mod pi_gpio;

use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

use anyhow::Result;
use audio::play_audio;
use bluetooth::rssi_by_inquiry;
use models::Item;
use pi_gpio::gpio_test;
use pino_utils::ok_or_continue_msg;

use crate::camera::take_picture;

#[tokio::main]
async fn main() {
    let (motion_tx, motion_rx) = mpsc::channel::<Vec<u8>>();
    let (gpio_tx, gpio_rx) = mpsc::channel::<Item>();

    let opencv_handle = thread::spawn(move || {
        if let Err(e) = motion::opencv_test(motion_tx, 2) {
            println!("{:?}", e);
        }
    });
    let gpio_handle = thread::spawn(move || {
        for recv in gpio_rx {
            println!("motor for item {:?}", recv);
        }
    });

    // might not need handles to each audio thread
    let mut audio_thread_pool: Vec<JoinHandle<()>> = Vec::new();
    for recv in motion_rx {
        /*
        let device_name = ok_or_continue_msg!(rssi_by_inquiry().await, |e| {
            println!("{:?}", e);
        });
        let resp = ok_or_continue_msg!(api::classify("nithin", recv), |e| {
            // println!("{:?}", e);
        });
        if gpio_tx.send(resp.item_type).is_err() {
            println!("Error sending to gpio thread");
        }
        */
        let audio_handle = thread::spawn(move || {
            play_audio().unwrap();
        });
        audio_thread_pool.push(audio_handle);
    }

    opencv_handle.join().unwrap();
}
