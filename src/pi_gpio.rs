use std::{thread, time::Duration};

use anyhow::Result;
use pino_utils::ok_or_continue;
use sysfs_gpio::{Direction, Pin};

const LOW: u8 = 1;
const HIGH: u8 = 0;

const DUR: u64 = 200;
const GAP: u64 = 400;
const LARGE_GAP: u64 = 500;
const WAIT: u64 = 1500;

/// Flash the LED for a set number of flashes
pub fn flash(flashes: u64) -> Result<()> {
    const FLASH_PERIOD: u64 = 100;

    let gpio25 = Pin::new(25);
    gpio25.set_direction(Direction::Out)?;

    for i in 0..flashes {
        gpio25.set_value(LOW)?;
        thread::sleep(Duration::from_millis(FLASH_PERIOD));
        gpio25.set_value(HIGH)?;
        thread::sleep(Duration::from_millis(FLASH_PERIOD));
    }
    Ok(())
}

/// Rotate pin a given number of steps counter-clockwise, then the same number of steps clockwise
pub fn rotate(steps: u64, reverse: bool) -> Result<()> {
    let gpio23 = Pin::new(23);
    let gpio24 = Pin::new(24);
    gpio23.set_direction(Direction::Out)?;
    gpio24.set_direction(Direction::Out)?;

    let motor_action = move |gpio23_val: u8, gpio24_val: u8, dur: u64| -> Result<()> {
        gpio23.set_value(gpio23_val)?; // clockwise
        gpio24.set_value(gpio24_val)?; // counterclockwise
        thread::sleep(Duration::from_millis(dur));
        Ok(())
    };

    for i in 0..steps {
        if reverse {
            motor_action(LOW, HIGH, DUR)?;
        } else {
            motor_action(HIGH, LOW, DUR)?;
        }
        motor_action(LOW, LOW, GAP)?;
    }

    motor_action(LOW, LOW, WAIT)?;

    for i in 0..steps {
        if reverse {
            motor_action(HIGH, LOW, DUR)?;
        } else {
            motor_action(LOW, HIGH, DUR)?;
        }
        motor_action(LOW, LOW, GAP)?;
    }

    Ok(())
}

/// Rotate a number of steps counter-clockwise
pub fn right(steps: u64) -> Result<()> {
    let gpio23 = Pin::new(23);
    let gpio24 = Pin::new(24);
    gpio23.set_direction(Direction::Out)?;
    gpio24.set_direction(Direction::Out)?;

    let motor_action = move |gpio23_val: u8, gpio24_val: u8, dur: u64| -> Result<()> {
        gpio23.set_value(gpio23_val)?; // clockwise
        gpio24.set_value(gpio24_val)?; // counterclockwise
        thread::sleep(Duration::from_millis(dur));
        Ok(())
    };

    for i in 0..steps {
        motor_action(HIGH, LOW, DUR)?;
        motor_action(LOW, LOW, GAP)?;
    }

    Ok(())
}

/// Rotate a number of steps clockwise
pub fn left(steps: u64) -> Result<()> {
    let gpio23 = Pin::new(23);
    let gpio24 = Pin::new(24);
    gpio23.set_direction(Direction::Out)?;
    gpio24.set_direction(Direction::Out)?;

    let motor_action = move |gpio23_val: u8, gpio24_val: u8, dur: u64| -> Result<()> {
        gpio23.set_value(gpio23_val)?; // clockwise
        gpio24.set_value(gpio24_val)?; // counterclockwise
        thread::sleep(Duration::from_millis(dur));
        Ok(())
    };

    for i in 0..steps {
        motor_action(LOW, HIGH, DUR)?;
        motor_action(LOW, LOW, GAP)?;
    }

    Ok(())
}

/// Force all pins to low
pub fn disable() -> Result<()> {
    let gpio23 = Pin::new(23);
    let gpio24 = Pin::new(24);
    gpio23.set_direction(Direction::Out)?;
    gpio24.set_direction(Direction::Out)?;
    gpio23.set_value(LOW)?;
    gpio24.set_value(LOW)?;
    Ok(())
}
