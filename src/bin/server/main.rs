use std::{env, sync::mpsc, thread};

use garbagio_midend::{
    api, audio::play_audio, bluetooth::rssi_by_inquiry, models::Item, motion, pi_gpio,
};
use log::error;
use pino_utils::ok_or_continue_msg;

#[tokio::main]
async fn main() {
    env_logger::builder().format_timestamp(None).init();

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
            error!("[OPENCV ERROR] {:?}", e);
        }
    });

    // gpio thread
    let _gpio_handle = thread::spawn(move || {
        for recv in gpio_rx {
            // convert item to a duration
            let steps = match recv {
                Item::Garbage => 0,
                Item::Blue => 4,
                Item::Red => 8,
            };
            if let Err(e) = pi_gpio::rotate(steps) {
                error!("[GPIO ERROR] {:?}", e);
            }
        }
    });

    // main thread handles bluetooth discovery
    for recv in motion_rx {
        let devices = match rssi_by_inquiry().await {
            Ok(device_name) => device_name,
            Err(e) => {
                error!("[BLUETOOTH ERROR] {:?}", e);
                Vec::new()
            },
        };
        let resp = ok_or_continue_msg!(api::classify(devices, recv).await, |e| {
            error!("[API ERROR] {:?}", e);
        });

        gpio_tx.send(resp.item_type).unwrap();

        let _audio_handle = thread::spawn(move || {
            let decoded = base64::decode(resp.audio).unwrap();
            if let Err(e) = play_audio(decoded) {
                error!("[AUDIO ERROR] {:?}", e);
            }
        });
    }

    opencv_handle.join().unwrap();
}
