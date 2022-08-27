use std::{thread, time::Duration};

use anyhow::Result;
use gpio::{sysfs::*, GpioIn, GpioOut};
use pino_utils::ok_or_continue;

pub fn gpio_test() -> Result<()> {
    let mut gpio23 = SysFsGpioOutput::open(23)?;
    let mut gpio24 = SysFsGpioOutput::open(24)?;

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
            ok_or_continue!(gpio23.set_value(gpio23_val));
            ok_or_continue!(gpio24.set_value(!gpio23_val));
        } else {
            ok_or_continue!(gpio23.set_value(!gpio23_val));
            ok_or_continue!(gpio24.set_value(gpio23_val));
        }

        thread::sleep(Duration::from_millis(1000));
        ok_or_continue!(gpio23.set_value(false));
        ok_or_continue!(gpio24.set_value(false));
        thread::sleep(Duration::from_millis(500));
        gpio23_val = !gpio23_val;
    }

    Ok(())
}
