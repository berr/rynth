use crate::basic::{AudioComponent, ModulationComponent, ModulationId};

pub type AudioComponents = Vec<Box<dyn AudioComponent + Send>>;
pub type ModulationComponents = Vec<Box<dyn ModulationComponent + Send>>;

pub struct Engine {
    current_audio_sample: u64,
    current_modulation_sample: u64,
    last_audio_sample_with_modulation: u64,
    pub sampling_rate: u32,
    pub modulation_rate: u32,
    pub channels: u16,
    components: AudioComponents,
    modulators: ModulationComponents,
}

impl Engine {
    pub fn new(sampling_rate: u32, modulation_rate: u32, channels: u16, components: AudioComponents, modulators: ModulationComponents) -> Self {
        Self {
            current_audio_sample: 0,
            current_modulation_sample: 0,
            last_audio_sample_with_modulation: 0,
            sampling_rate,
            modulation_rate,
            channels,
            components,
            modulators,
        }
    }

    pub fn advance(&mut self, audio: &mut [f32]) {
        let channels = self.channels as usize;
        let total_samples = audio.len() / channels;
        let start_sample = self.current_audio_sample;
        let end_sample = start_sample + total_samples as u64;
        assert_eq!(total_samples * channels, audio.len());

        println!("Advancing from {} to {}", start_sample, end_sample);

        if self.current_audio_sample == 0 {
            self.process_modulation();
        }

        let samples_before_next_modulation = (self.last_audio_sample_with_modulation + self.modulation_rate as u64 - start_sample) as usize;

        if samples_before_next_modulation > total_samples {
            self.process_audio(audio, total_samples);
            return;
        }

        self.process_audio(&mut audio[0..channels*samples_before_next_modulation], samples_before_next_modulation);
        let mut samples_remaining = total_samples - samples_before_next_modulation;


        let mut current_sample_start_offset = samples_before_next_modulation;
        let audio_samples = self.modulation_rate as usize;

        while samples_remaining > 0 {
            self.process_modulation();
            let samples_to_process = samples_remaining.min(current_sample_start_offset + audio_samples);
            let current_sample_end_offset = current_sample_start_offset + samples_to_process;
            self.process_audio(&mut audio[current_sample_start_offset*channels..current_sample_end_offset*channels], samples_to_process);
            current_sample_start_offset = current_sample_end_offset;

            samples_remaining -= samples_to_process;
        }
    }

    fn process_modulation(&mut self) {
        println!("Processing modulation");
        for m in &mut self.modulators {
            m.process_modulation(self.current_modulation_sample);
        }

        self.last_audio_sample_with_modulation = self.current_audio_sample;
        self.current_modulation_sample += 1;
    }

    fn process_audio(&mut self, audio: &mut [f32], total_samples: usize) {
        if total_samples == 0 {
            return;
        }

        println!("Processing audio");
        let start_sample = self.current_audio_sample;
        let end_sample = start_sample + total_samples as u64;

        for c in &mut self.components {
            c.process_audio(audio, start_sample..end_sample);
        }

        self.current_audio_sample = end_sample;
    }
}


