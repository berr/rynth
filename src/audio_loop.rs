use cpal::traits::{DeviceTrait, StreamTrait};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use crate::engine::Engine;

pub type Message = ();
pub type CommandReceiver = Receiver<Message>;
pub type CommandSender = Sender<Message>;


pub fn audio_loop(mut engine: Engine, device: cpal::Device, receiver: CommandReceiver) -> Result<(), anyhow::Error>
{
    let config = cpal::StreamConfig{
        channels: engine.channels,
        sample_rate: cpal::SampleRate(engine.sample_rate),
        buffer_size: cpal::BufferSize::Fixed(128)
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            engine.process_audio(data);
        },
        err_fn,
    )?;

    thread::sleep(Duration::from_millis(100));

    stream.play()?;

    while let Ok(_) = receiver.recv() {
        println!("Received something? wtf");
    }

    stream.pause()?;

    Ok(())
}