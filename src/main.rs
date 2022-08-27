#![allow(unused)]
#![allow(dead_code)]

mod api;
mod bluetooth;
mod camera;
mod pi_gpio;

use pi_gpio::gpio_test;

use crate::camera::take_picture;
use bluetooth::rssi_by_inquiry;

#[tokio::main(flavor = "current_thread")]
async fn main(){

}
