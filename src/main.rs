use anyhow::Result;
use rynth::app::{audio_loop, configure_device, create_demo_engine};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let device = configure_device()?;
    let (engine, topology) = create_demo_engine();

    let (tx, rx) = channel();

    let handle = thread::spawn(move || audio_loop(engine, topology, device, rx));
    thread::sleep(Duration::from_millis(10000));

    drop(tx);
    handle.join().unwrap().unwrap();

    Ok(())
}
