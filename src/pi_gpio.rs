use std::{thread, time::Duration};

use anyhow::Result;
use pino_utils::ok_or_continue;
use sysfs_gpio::Pin;

pub fn gpio_test() -> Result<()> {
    let mut gpio23 = Pin::new(23);
    let mut gpio24 = Pin::new(24);

    // print value of gpio 23
    /*
    loop {
        let gpio23_val = ok_or_continue!(gpio23.read_value());
        println!("GPIO23: {:?}", gpio23_val);
        thread::sleep(Duration::from_millis(1000));
    }
    */
    let mut gpio23_val = false;
    loop {
        if gpio23_val {
            ok_or_continue!(gpio23.set_value(gpio23_val as u8));
            ok_or_continue!(gpio24.set_value(!gpio23_val as u8));
        } else {
            ok_or_continue!(gpio23.set_value(!gpio23_val as u8));
            ok_or_continue!(gpio24.set_value(gpio23_val as u8));
        }

        thread::sleep(Duration::from_millis(1000));
        ok_or_continue!(gpio23.set_value(false as u8));
        ok_or_continue!(gpio24.set_value(false as u8));
        thread::sleep(Duration::from_millis(500));
        gpio23_val = !gpio23_val;
    }

    Ok(())
}
