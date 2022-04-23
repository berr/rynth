use std::time::Duration;
use anyhow::Result;
use rynth::basic::AudioComponent;
use rynth::engine::Engine;
use rynth::oscillator::Oscillator;
use hound::WavWriter;

fn save_engine_result(engine: &mut Engine, duration: Duration, path: &str) -> Result<()> {
    let samples_per_call = 128;
    let channels = engine.channels;
    let sample_rate = engine.sample_rate;
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

        engine.process_audio(buffer_slice);
        for s in buffer_slice.iter() {
            writer.write_sample(*s);
        }

        current_sample += samples_in_loop;
    }

    Ok(())
}

fn create_testing_engine() -> Result<Engine> {
    let sample_rate = 48000;

    let sine_oscillator: Box<dyn AudioComponent + Send + 'static> = Box::new(Oscillator::new(440.0, sample_rate));
    let components = vec![sine_oscillator];
    let modulators = vec![];


    let engine = Engine::new(
        sample_rate,
        2,
        components,
        modulators,
    );

    Ok(engine)
}

#[test]
fn sine_wave() -> Result<()> {
    let mut engine = create_testing_engine()?;
    save_engine_result(&mut engine, Duration::from_millis(100), "sine.wav")?;

    Ok(())
}