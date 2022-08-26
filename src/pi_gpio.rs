use anyhow::Result;
use gpio::{GpioIn, GpioOut, sysfs::*};
use std::{thread, time::Duration};
use pino_utils::ok_or_continue;

pub fn gpio_test() -> Result<()> {
    let mut gpio23 = SysFsGpioInput::open(23)?;

    loop {
        let gpio23_val = ok_or_continue!(gpio23.read_value());
        println!("GPIO23: {:?}", gpio23_val);
        thread::sleep(Duration::from_millis(1000));
    }

    Ok(())
}

