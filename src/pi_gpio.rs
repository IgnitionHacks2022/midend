use std::{thread, time::Duration};

use anyhow::Result;
use gpio::{sysfs::*, GpioIn, GpioOut};
use pino_utils::ok_or_continue;

pub fn gpio_test() -> Result<()> {
    let mut gpio23 = SysFsGpioOutput::open(23)?;

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
        ok_or_continue!(gpio23.set_value(gpio23_val));
        thread::sleep(Duration::from_millis(1000));
        gpio23_val = !gpio23_val;
    }

    Ok(())
}
