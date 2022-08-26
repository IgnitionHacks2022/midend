#![allow(unused)]
#![allow(dead_code)]

mod api;
mod bluetooth;
mod camera;
mod pi_gpio;

use pi_gpio::gpio_test;

use crate::camera::take_picture;

fn main() {
    // take_picture("/dev/video2");
    gpio_test().unwrap();
}
