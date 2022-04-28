use anyhow::Result;
use rynth::app::create_demo_engine;
use rynth::engine::{AudioTopology, Engine};
use std::time::Duration;

fn save_engine_result(
    engine: &mut Engine,
    topology: &mut AudioTopology,
    duration: Duration,
    path: &str,
) -> Result<()> {
    let samples_per_call = 512;
    let channels = engine.channels;
    let sampling_rate = engine.sampling_rate;
    let test_samples = (duration.as_secs_f32() * sampling_rate.0 as f32) as usize;
    let mut buffer = vec![0.0; samples_per_call * channels.0 as usize];

    let spec = hound::WavSpec {
        channels: channels.0,
        sample_rate: sampling_rate.0,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;

    let mut current_sample = 0;
    while current_sample < test_samples {
        let samples_in_loop = samples_per_call.min(test_samples - current_sample);
        let samples_in_buffer = samples_in_loop * channels.0 as usize;
        let buffer_slice = &mut buffer.as_mut_slice()[0..samples_in_buffer];

        engine.advance(topology, buffer_slice);
        for s in buffer_slice.iter() {
            writer.write_sample(*s)?;
        }

        current_sample += samples_in_loop;
    }

    Ok(())
}

#[test]
fn sine_wave() -> Result<()> {
    let (mut engine, mut topology) = create_demo_engine();
    save_engine_result(
        &mut engine,
        &mut topology,
        Duration::from_millis(10000),
        "sine.wav",
    )?;

    Ok(())
}
