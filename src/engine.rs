use crate::basic::AudioComponent;

pub type Components = Vec<Box<dyn AudioComponent + Send>>;

pub struct Engine {
    current_sample: u64,
    pub sample_rate: u32,
    pub channels: u16,
    components: Vec<Box<dyn AudioComponent + Send>>,
    modulators: Vec<Box<dyn AudioComponent + Send>>,
}

impl Engine {
    pub fn new(sample_rate: u32, channels: u16, components: Components, modulators: Components) -> Self {
        Self {
            current_sample: 0,
            sample_rate,
            channels,
            components,
            modulators,
        }
    }

    pub fn process_audio(&mut self, audio: &mut [f32]) {
        let samples = (audio.len() / self.channels as usize) as u64;
        assert_eq!(samples * self.channels as u64, audio.len() as u64);

        let start_sample = self.current_sample;
        let end_sample = start_sample + samples;

        for c in &mut self.components {
            c.process_audio(audio, start_sample..end_sample);
        }

        self.current_sample = end_sample;
    }
}


