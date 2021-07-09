use log::{info, debug, LevelFilter};
use std::time::Duration;
use std::thread::sleep;


fn output_logo() {
    info!("     _________");
    info!("    /  /       \\");
    info!("   /  /  /  /  /   Undermountain MUD Engine");
    info!("   \\____/__/__/");
    info!("")
}

fn setup_logger() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::max())
        .init();
}

fn main() {
    setup_logger();

    output_logo();

    info!("Starting up.");
    // TODO: Initialize managers/modules.

    loop {
        // TODO: Emit a tick event down the channels.
        debug!("Tick.");
        // TODO: Make the tick duration configurable?
        sleep(Duration::from_millis(1000))
    }
}
