mod helpers;
mod resource_db;

use anyhow::Result;
use helpers::assert_engine_produces_same_output;
use resource_db::get_resource;
use rynth::components::{LowFrequencyOscillator, Oscillator};
use rynth::core::{empty_engine, AudioTopology, Channels, Engine, ModulationRate, SamplingRate};
use std::time::Duration;

fn empty_mono_engine() -> (Engine, AudioTopology) {
    empty_engine(SamplingRate(48000), ModulationRate(100), 128, Channels(1))
}

#[test]
fn simple_oscillator() -> Result<()> {
    let (mut engine, mut topology) = empty_mono_engine();

    let mut oscillator = Oscillator::new(400.0, engine.spec.sampling_rate);
    oscillator.level.set_value(0.75);
    topology.add_component(oscillator);

    assert_engine_produces_same_output(
        &mut engine,
        &mut topology,
        Duration::from_millis(5000),
        &get_resource("sine.wav"),
    )?;

    Ok(())
}

#[test]
fn oscillator_amplitude_modulation() -> Result<()> {
    let (mut engine, mut topology) = empty_mono_engine();

    let modulator_id = topology.add_modulator(LowFrequencyOscillator::new(
        2.0,
        engine.spec.modulation_rate,
    ));

    let mut oscillator = Oscillator::new(500.0, engine.spec.sampling_rate);

    oscillator.level.set_value(0.3);
    oscillator.level.add_modulation(modulator_id, 0.2);

    topology.add_component(oscillator);

    assert_engine_produces_same_output(
        &mut engine,
        &mut topology,
        Duration::from_millis(5000),
        &get_resource("sine_amplitude_modulation.wav"),
    )?;

    Ok(())
}

#[test]
fn oscillator_frequency_modulation() -> Result<()> {
    let (mut engine, mut topology) = empty_mono_engine();

    let modulator_id = topology.add_modulator(LowFrequencyOscillator::new(
        10.0,
        engine.spec.modulation_rate,
    ));

    let mut oscillator = Oscillator::new(1500.0, engine.spec.sampling_rate);

    oscillator.level.set_value(0.75);
    oscillator.frequency.add_modulation(modulator_id, 0.05); // 1000Hz range

    topology.add_component(oscillator);

    assert_engine_produces_same_output(
        &mut engine,
        &mut topology,
        Duration::from_millis(5000),
        &get_resource("sine_frequency_modulation.wav"),
    )?;

    Ok(())
}

#[test]
fn oscillator_frequency_and_amplitude_modulation() -> Result<()> {
    let (mut engine, mut topology) = empty_mono_engine();

    let modulator1_id = topology.add_modulator(LowFrequencyOscillator::new(
        2.0,
        engine.spec.modulation_rate,
    ));
    let modulator2_id = topology.add_modulator(LowFrequencyOscillator::new(
        10.0,
        engine.spec.modulation_rate,
    ));

    let mut oscillator = Oscillator::new(500.0, engine.spec.sampling_rate);

    oscillator.level.set_value(0.3);
    oscillator.level.add_modulation(modulator1_id, 0.2);
    oscillator.frequency.add_modulation(modulator2_id, 0.01); // 200Hz range

    topology.add_component(oscillator);

    assert_engine_produces_same_output(
        &mut engine,
        &mut topology,
        Duration::from_millis(5000),
        &get_resource("sine_frequency_and_amplitude_modulation.wav"),
    )?;

    Ok(())
}
