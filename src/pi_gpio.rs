use std::{thread, time::Duration};

use anyhow::Result;
use pino_utils::ok_or_continue;
use sysfs_gpio::{Direction, Pin};

pub fn gpio_test() -> Result<()> {
    let gpio23 = Pin::new(23);
    let gpio24 = Pin::new(24);
    gpio23.set_direction(Direction::Out)?;
    gpio24.set_direction(Direction::Out)?;
    loop {
        println!("zeroing");
        gpio23.set_value(1)?;
        gpio24.set_value(1)?;
        thread::sleep(Duration::from_millis(200));
        println!("enabling gpio23");
        gpio23.set_value(1)?;
        gpio24.set_value(0)?;
        thread::sleep(Duration::from_millis(1000));
        println!("zeroing");
        gpio23.set_value(1)?;
        gpio24.set_value(1)?;
        thread::sleep(Duration::from_millis(200));
        println!("enabling gpio24");
        gpio23.set_value(0)?;
        gpio24.set_value(1)?;
        thread::sleep(Duration::from_millis(1000));
    }
}
