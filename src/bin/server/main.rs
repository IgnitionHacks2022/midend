use std::{env, sync::mpsc, thread};

use garbagio_midend::{
    api, audio::play_audio, bluetooth::rssi_by_inquiry, models::Item, motion, pi_gpio,
};
use pino_utils::ok_or_continue_msg;

#[tokio::main]
async fn main() {
    // read args
    let args = env::args().collect::<Vec<_>>();
    let video_index = args.get(1).and_then(|x| x.parse::<i32>().ok()).unwrap_or(0);
    let video_debug = args
        .get(2)
        .and_then(|x| x.parse::<bool>().ok())
        .unwrap_or(false);

    // intialize channels
    let (motion_tx, motion_rx) = mpsc::channel::<Vec<u8>>();
    let (gpio_tx, gpio_rx) = mpsc::channel::<Item>();

    // motion detection thread
    let opencv_handle = thread::spawn(move || {
        if let Err(e) = motion::motion_detection(motion_tx, video_index, video_debug) {
            println!("[OPENCV ERROR] {:?}", e);
        }
    });

    // gpio thread
    let _gpio_handle = thread::spawn(move || {
        for recv in gpio_rx {
            // convert item to a duration
            let steps = match recv {
                Item::Garbage => 3,
                Item::Blue => 5,
                Item::Red => 7,
            };
            if let Err(e) = pi_gpio::rotate(steps) {
                println!("[GPIO ERROR] {:?}", e);
            }
        }
    });

    // main thread handles bluetooth discovery
    for recv in motion_rx {
        let device_name = match rssi_by_inquiry().await {
            Ok(device_name) => device_name,
            Err(e) => {
                println!("[BLUETOOTH ERROR] {:?}", e);
                String::from("None")
            },
        };
        let resp = ok_or_continue_msg!(api::classify(device_name, recv).await, |e| {
            println!("[API ERROR] {:?}", e);
        });

        let item_type = Item::try_from(resp.item_type).unwrap_or(Item::Garbage);
        gpio_tx.send(item_type).unwrap();

        let _audio_handle = thread::spawn(move || {
            let decoded = base64::decode(resp.audio).unwrap();
            if let Err(e) = play_audio(decoded) {
                println!("[AUDIO ERROR] {:?}", e);
            }
        });
    }

    opencv_handle.join().unwrap();
}
