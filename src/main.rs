// src/main.rs

mod osc_listener;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use osc_listener::start_osc_listener;

fn main() {
    // Initialize logger if needed.
    env_logger::init();

    // Channel for OSC messages.
    let (osc_tx, osc_rx) = mpsc::channel();

    // Spawn the OSC listener on a separate thread.
    thread::spawn(move || {
        start_osc_listener(osc_tx);
    });

    // A simple loop that prints received OSC messages.
    loop {
        while let Ok(osc_message) = osc_rx.try_recv() {
            println!("Received OSC message: {:?}", osc_message);
        }
        // Sleep a bit so we don't spin too fast.
        thread::sleep(Duration::from_millis(16));
    }
}
