use anyhow::Result;
use rynth::engine::{empty_engine, AudioTopology, Engine};
use rynth::low_frequency_oscillator::LowFrequencyOscillator;
use rynth::oscillator::Oscillator;
use std::time::Duration;

fn save_engine_result(
    engine: &mut Engine,
    topology: &mut AudioTopology,
    duration: Duration,
    path: &str,
) -> Result<()> {
    let samples_per_call = 512;
    let channels = engine.channels;
    let sample_rate = engine.sampling_rate;
    let test_samples = (duration.as_secs_f32() * sample_rate as f32) as usize;
    let mut buffer = vec![0.0; samples_per_call * channels as usize];

    let spec = hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;

    let mut current_sample = 0;
    while current_sample < test_samples {
        let samples_in_loop = samples_per_call.min(test_samples - current_sample);
        let samples_in_buffer = samples_in_loop * channels as usize;
        let buffer_slice = &mut buffer.as_mut_slice()[0..samples_in_buffer];

        engine.advance(topology, buffer_slice);
        for s in buffer_slice.iter() {
            writer.write_sample(*s)?;
        }

        current_sample += samples_in_loop;
    }

    Ok(())
}

fn create_testing_engine() -> (Engine, AudioTopology) {
    let sampling_rate = 48000;
    let modulation_rate = 100;
    let modulation_interval = sampling_rate / modulation_rate;

    let (engine, mut topology) = empty_engine(sampling_rate, modulation_interval, 2);

    let modulator_id = topology.add_modulator(LowFrequencyOscillator::new(2.0, modulation_rate));

    let mut oscillator = Oscillator::new(200.0, sampling_rate);
    oscillator.level.value = 0.5;
    oscillator.level.add_modulation(modulator_id, 0.4);

    // oscillator.frequency.add_modulation(modulator_id, 0.002);

    topology.add_component(oscillator);

    (engine, topology)
}

#[test]
fn sine_wave() -> Result<()> {
    let (mut engine, mut topology) = create_testing_engine();
    save_engine_result(
        &mut engine,
        &mut topology,
        Duration::from_millis(10000),
        "sine.wav",
    )?;

    Ok(())
}
