use std::{thread, time::Duration};

use anyhow::Result;
use pino_utils::ok_or_continue;
use sysfs_gpio::{Direction, Pin};

const LOW: u8 = 1;
const HIGH: u8 = 0;

pub fn rotate(steps: u64) -> Result<()> {
    let gpio23 = Pin::new(23);
    let gpio24 = Pin::new(24);
    gpio23.set_direction(Direction::Out)?;
    gpio24.set_direction(Direction::Out)?;

    let motor_action = move |gpio23_val: u8, gpio24_val: u8, dur: u64| -> Result<()> {
        gpio23.set_value(gpio23_val)?;
        gpio24.set_value(gpio24_val)?;
        thread::sleep(Duration::from_millis(dur));
        Ok(())
    };

    const DUR: u64 = 200;
    const GAP: u64 = 400;
    const LARGE_GAP: u64 = 500;

    loop {
        for i in 0..steps {
            motor_action(LOW, LOW, GAP)?;
            motor_action(HIGH, LOW, DUR)?;
        }
        motor_action(LOW, LOW, LARGE_GAP)?;

        for i in 0..steps {
            motor_action(LOW, LOW, GAP)?;
            motor_action(LOW, HIGH, DUR)?;
        }
        motor_action(LOW, LOW, LARGE_GAP)?;
    }

    Ok(())
}

pub fn disable() -> Result<()> {
    let gpio23 = Pin::new(23);
    let gpio24 = Pin::new(24);
    gpio23.set_direction(Direction::Out)?;
    gpio24.set_direction(Direction::Out)?;
    println!("zeroing");
    gpio23.set_value(1)?;
    gpio24.set_value(1)?;
    Ok(())
}

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
        thread::sleep(Duration::from_millis(5000));
        println!("zeroing");
        gpio23.set_value(1)?;
        gpio24.set_value(1)?;
        thread::sleep(Duration::from_millis(200));
        println!("enabling gpio24");
        gpio23.set_value(0)?;
        gpio24.set_value(1)?;
        thread::sleep(Duration::from_millis(5000));
    }
}
