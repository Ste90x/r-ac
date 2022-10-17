use std::{env, thread, time};

use enigo::{Enigo, MouseButton, MouseControllable};

fn main() {
    let mut enigo = Enigo::new();

    let args: Vec<String> = env::args().collect();
    let parsed_interval = args[1].parse::<u64>().unwrap();

    let interval_in_ms = parsed_interval * 60 * 1000;

    println!(
        "Starting auto clicker with an interval of {} minutes ...",
        parsed_interval
    );

    let interval = time::Duration::from_millis(interval_in_ms);

    loop {
        enigo.mouse_click(MouseButton::Left);
        println!("Clicked! Waiting for {} minutes", parsed_interval);
        thread::sleep(interval);
    }
}
