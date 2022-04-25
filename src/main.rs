use rynth::app::configure_device;
use rynth::audio_loop::audio_loop;
use rynth::engine::{AudioTopology, empty_engine, Engine};
use anyhow::Result;

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use rynth::oscillator::Oscillator;


fn create_testing_engine() -> Result<(Engine, AudioTopology)> {
    let sampling_rate = 48000;
    let modulation_rate = sampling_rate / 100;

    let (engine, mut topology) = empty_engine(sampling_rate, modulation_rate, 2);
    topology.add_component(Oscillator::new(200.0, sampling_rate));

    Ok((engine, topology))
}


fn main() -> Result<()> {
    let device = configure_device()?;
    let (engine, topology) = create_testing_engine()?;

    let (tx, rx) = channel();

    let handle = thread::spawn(move || audio_loop(engine, topology, device, rx));
    thread::sleep(Duration::from_millis(10000));

    drop(tx);
    handle.join().unwrap().unwrap();

    Ok(())
}


