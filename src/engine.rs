use std::num::NonZeroUsize;
use crate::basic::{AudioComponent, AudioComponentId, ModulationComponent, ModulatorId};

pub type AudioComponents = Vec<Box<dyn AudioComponent + Send>>;
pub type ModulationComponents = Vec<Box<dyn ModulationComponent + Send>>;

pub struct Engine {
    current_audio_sample: u64,
    current_modulation_sample: u64,
    last_audio_sample_with_modulation: u64,
    pub sampling_rate: u32,
    pub modulation_rate: u32,
    pub channels: u16,
}

pub struct AudioTopology {
    components: AudioComponents,
    generated_components: usize,
    modulators: ModulationComponents,
    generated_modulators: usize,
}

impl AudioTopology {

    pub fn new() -> Self {
        Self{
            components: vec![],
            generated_components: 0,
            modulators: vec![],
            generated_modulators: 0
        }
    }

    pub fn add_component<T: AudioComponent + Send + 'static>(&mut self, mut component: T) -> AudioComponentId {
        self.generated_components += 1;
        let id = AudioComponentId(NonZeroUsize::new(self.generated_components).unwrap());
        component.change_id(id);

        self.components.push(Box::new(component));

        id
    }

    pub fn add_modulator<T: ModulationComponent + Send + 'static>(&mut self, mut modulator: T) -> ModulatorId {
        self.generated_modulators += 1;
        let id = ModulatorId(NonZeroUsize::new(self.generated_modulators).unwrap());
        modulator.change_id(id);

        self.modulators.push(Box::new(modulator));

        id
    }

}

impl Engine {
    pub fn new(sampling_rate: u32, modulation_rate: u32, channels: u16) -> Self {
        Self {
            current_audio_sample: 0,
            current_modulation_sample: 0,
            last_audio_sample_with_modulation: 0,
            sampling_rate,
            modulation_rate,
            channels,
        }
    }

    pub fn advance(&mut self, topology: &mut AudioTopology, audio: &mut [f32]) {
        let channels = self.channels as usize;
        let total_samples = audio.len() / channels;
        let start_sample = self.current_audio_sample;
        let end_sample = start_sample + total_samples as u64;
        assert_eq!(total_samples * channels, audio.len());

        println!("Advancing from {} to {}", start_sample, end_sample);

        if self.current_audio_sample == 0 {
            self.process_modulation(topology);
        }

        let samples_before_next_modulation = (self.last_audio_sample_with_modulation + self.modulation_rate as u64 - start_sample) as usize;

        if samples_before_next_modulation > total_samples {
            self.process_audio(topology, audio, total_samples);
            return;
        }

        self.process_audio(topology, &mut audio[0..channels*samples_before_next_modulation], samples_before_next_modulation);
        let mut samples_remaining = total_samples - samples_before_next_modulation;


        let mut current_sample_start_offset = samples_before_next_modulation;
        let audio_samples = self.modulation_rate as usize;

        while samples_remaining > 0 {
            self.process_modulation(topology);
            let samples_to_process = samples_remaining.min(current_sample_start_offset + audio_samples);
            let current_sample_end_offset = current_sample_start_offset + samples_to_process;
            self.process_audio(topology, &mut audio[current_sample_start_offset*channels..current_sample_end_offset*channels], samples_to_process);
            current_sample_start_offset = current_sample_end_offset;

            samples_remaining -= samples_to_process;
        }
    }

    fn process_modulation(&mut self, topology: &mut AudioTopology) {
        println!("Processing modulation");
        for m in &mut topology.modulators {
            m.process_modulation(self.current_modulation_sample);
        }

        self.last_audio_sample_with_modulation = self.current_audio_sample;
        self.current_modulation_sample += 1;
    }

    fn process_audio(&mut self, topology: &mut AudioTopology, audio: &mut [f32], total_samples: usize) {
        if total_samples == 0 {
            return;
        }

        println!("Processing audio");
        let start_sample = self.current_audio_sample;
        let end_sample = start_sample + total_samples as u64;

        for c in &mut topology.components {
            c.process_audio(audio, start_sample..end_sample);
        }

        self.current_audio_sample = end_sample;
    }
}


pub fn empty_engine(sampling_rate: u32, modulation_interval: u32, channels: u16) -> (Engine, AudioTopology) {
    let engine = Engine::new(sampling_rate, modulation_interval, channels);
    let audio_topology = AudioTopology::new();

    (engine, audio_topology)
}