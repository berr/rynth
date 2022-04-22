use rynth::app::configure_device;
use rynth::audio_loop::audio_loop;
use rynth::engine::Engine;
use anyhow::Result;

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use rynth::basic::AudioComponent;
use rynth::oscillator::Oscillator;


fn create_testing_engine() -> Result<Engine> {
    let sample_rate = 48000;

    let sine_oscillator: Box<dyn AudioComponent + Send + 'static> = Box::new(Oscillator::new(200.0, sample_rate));
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


fn main() -> Result<()> {
    let device = configure_device()?;
    let engine = create_testing_engine()?;

    let (tx, rx) = channel();

    let handle = thread::spawn(move || audio_loop(engine, device, rx));
    thread::sleep(Duration::from_millis(10000));

    drop(tx);
    handle.join().unwrap().unwrap();

    Ok(())
}


