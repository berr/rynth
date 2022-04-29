use anyhow::{bail, Result};
use rynth::core::{AudioTopology, Engine};
use std::iter::FromIterator;
use std::path::Path;
use std::time::Duration;

fn save_engine_result(
    engine: &mut Engine,
    topology: &mut AudioTopology,
    duration: Duration,
    path: &Path,
) -> Result<()> {
    let samples_per_call = engine.spec.max_samples_per_step;
    let channels = engine.spec.channels;
    let sampling_rate = engine.spec.sampling_rate;
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

pub fn assert_engine_produces_same_output(
    engine: &mut Engine,
    topology: &mut AudioTopology,
    duration: Duration,
    expected: &Path,
) -> Result<()> {
    let expected_filename = expected.file_name().unwrap().to_str().unwrap();
    let obtained_filename = format!("obtained_{}", expected_filename);
    let obtained = expected.with_file_name(obtained_filename);

    println!("Loading from {}", expected.to_str().unwrap());
    println!("Saving to {}", obtained.to_str().unwrap());
    save_engine_result(engine, topology, duration, &obtained).unwrap();

    compare_files(&obtained, expected)?;

    Ok(())
}

fn compare_files(obtained: &Path, expected: &Path) -> Result<()> {
    let (obtained_spec, obtained_data) = read_wave_file(obtained)?;
    let (expected_spec, expected_data) = read_wave_file(expected)?;

    assert_eq!(
        obtained_spec.channels,
        expected_spec.channels,
        "Different number of channels. Expected {}, obtained {}\nObtained file: {}",
        expected_spec.channels,
        obtained_spec.channels,
        obtained.to_str().unwrap()
    );

    assert_eq!(
        obtained_spec.sample_rate,
        expected_spec.sample_rate,
        "Different sample rate. Expected {}, obtained {}\nObtained file: {}",
        expected_spec.sample_rate,
        obtained_spec.sample_rate,
        obtained.to_str().unwrap()
    );

    assert_eq!(
        obtained_data,
        expected_data,
        "Obtained file: {}",
        obtained.to_str().unwrap()
    );

    std::fs::remove_file(obtained)?;

    Ok(())
}

fn read_wave_file(path: &Path) -> Result<(hound::WavSpec, Vec<f32>)> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();

    if spec.sample_format != hound::SampleFormat::Float {
        bail!("File didn't contain floats");
    }

    let samples = reader.samples::<f32>().map(|s| s.unwrap());
    let data = Vec::from_iter(samples);

    Ok((spec, data))
}
