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

/// Rotate pin a given number of steps counter-clockwise, then the same number of steps clockwise
pub fn rotate(steps: u64) -> Result<()> {
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

    motor_action(LOW, LOW, WAIT)?;

    for i in 0..steps {
        motor_action(LOW, HIGH, DUR)?;
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
