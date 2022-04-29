use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::FrameCount;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use crate::core::{AudioTopology, Engine};

pub type Message = ();
pub type CommandReceiver = Receiver<Message>;
pub type CommandSender = Sender<Message>;

pub fn audio_loop(
    mut engine: Engine,
    mut topology: AudioTopology,
    device: cpal::Device,
    receiver: CommandReceiver,
) -> Result<(), anyhow::Error> {
    let config = cpal::StreamConfig {
        channels: engine.spec.channels.0,
        sample_rate: cpal::SampleRate(engine.spec.sampling_rate.0),
        buffer_size: cpal::BufferSize::Fixed(engine.spec.max_samples_per_step as FrameCount),
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            engine.advance(&mut topology, data);
        },
        err_fn,
    )?;

    thread::sleep(Duration::from_millis(100));

    stream.play()?;

    while receiver.recv().is_ok() {
        println!("Received something? wtf");
    }

    stream.pause()?;

    Ok(())
}
