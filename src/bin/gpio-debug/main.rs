use std::io::{stdin, stdout, Write};

use garbagio_midend::pi_gpio::{disable, left, right};

fn main() {
    loop {
        // get user input
        let mut input = String::new();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_owned();

        if input.is_empty() {
            continue;
        }

        match input.chars().nth(0).unwrap() {
            'l' => {
                let (_, steps) = input.split_once(" ").unwrap_or(("", "1"));
                let steps = steps.parse::<u64>().unwrap_or(1);
                println!("stepping left {} steps...", steps);
                left(steps).unwrap();
            },
            'r' => {
                let (_, steps) = input.split_once(" ").unwrap_or(("", "1"));
                let steps = steps.parse::<u64>().unwrap_or(1);
                println!("stepping right {} steps...", steps);
                right(steps).unwrap();
            },
            'z' => {
                println!("resetting...");
                disable().unwrap();
            },
            'q' => break,
            _ => {},
        }
    }
}
