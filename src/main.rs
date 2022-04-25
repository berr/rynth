use anyhow::Result;
use rynth::app::configure_device;
use rynth::audio_loop::audio_loop;
use rynth::engine::{empty_engine, AudioTopology, Channels, Engine, ModulationRate, SamplingRate};

use rynth::low_frequency_oscillator::LowFrequencyOscillator;
use rynth::oscillator::Oscillator;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn create_testing_engine() -> Result<(Engine, AudioTopology)> {
    let sampling_rate = SamplingRate(48000);
    let modulation_rate = ModulationRate(100);

    let (engine, mut topology) = empty_engine(sampling_rate, modulation_rate, Channels(2));

    let modulator_id = topology.add_modulator(LowFrequencyOscillator::new(2.0, modulation_rate));

    let mut oscillator = Oscillator::new(200.0, sampling_rate);
    oscillator.level.value = 0.4;
    oscillator.level.add_modulation(modulator_id, 0.5);

    // oscillator.frequency.add_modulation(modulator_id, 0.002);

    topology.add_component(oscillator);

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
