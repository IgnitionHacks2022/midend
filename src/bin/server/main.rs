use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

use anyhow::Result;
use garbagio_midend::{
    api, audio::play_audio, bluetooth::rssi_by_inquiry, camera::take_picture, models::Item, motion, pi_gpio
};
use pino_utils::ok_or_continue_msg;

#[tokio::main]
async fn main() {
    // intialize channels
    let (motion_tx, motion_rx) = mpsc::channel::<Vec<u8>>();
    let (gpio_tx, gpio_rx) = mpsc::channel::<Item>();

    // motion detection thread
    let opencv_handle = thread::spawn(move || {
        motion::opencv_test(motion_tx, 0).unwrap();
    });

    // gpio thread
    let gpio_handle = thread::spawn(move || {
        for recv in gpio_rx {
            println!("motor for item {:?}", recv);

            // convert item to a duration
            let steps = match recv {
                Item::Garbage => 3,
                Item::Blue => 5,
                Item::Red => 7,
            };
            pi_gpio::rotate(steps);
        }
    });

    // main thread handles bluetooth discovery
    // might not need handles to each audio thread
    let mut audio_thread_pool: Vec<JoinHandle<()>> = Vec::new();
    for recv in motion_rx {
        /*
        let device_name = ok_or_continue_msg!(rssi_by_inquiry().await, |e| {
            println!("{:?}", e);
        });
        */
        let resp = ok_or_continue_msg!(api::classify("test", recv).await, |e| {
            println!("classify error {:?}", e);
        });

        let item_type = Item::try_from(resp.item_type).unwrap();

        if gpio_tx.send(item_type).is_err() {
            println!("Error sending to gpio thread");
        }
        let audio_handle = thread::spawn(move || {
            let decoded = base64::decode(resp.audio).unwrap();
            if let Err(e) = play_audio(decoded) {
                println!("Error playing audio {:?}", e);
            }
        });
    }

    opencv_handle.join().unwrap();
}
