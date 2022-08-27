#![allow(unused)]
#![allow(dead_code)]

mod api;
mod bluetooth;
mod camera;
mod pi_gpio;

use api::classify;
use pi_gpio::gpio_test;

use crate::camera::take_picture;

fn main() {
    let image = take_picture("/dev/video2").unwrap();
    classify("nithin", image).unwrap();
    // gpio_test().unwrap();
}
