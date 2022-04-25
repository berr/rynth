use crate::basic::{AudioComponent, AudioComponentId, ModulationComponent, ModulatorId};
use std::num::NonZeroUsize;

use derive_more::{Add, AddAssign, Sub, SubAssign};

pub type AudioComponents = Vec<Box<dyn AudioComponent + Send>>;
pub type ModulationComponents = Vec<Box<dyn ModulationComponent + Send>>;

#[derive(PartialEq, Copy, Clone)]
pub struct AudioSampleIndex(pub u64);

#[derive(Copy, Clone, Add, AddAssign, Sub, SubAssign, Ord, PartialOrd, Eq, PartialEq)]
pub struct AudioSampleDifference(pub u64);

impl std::ops::Add<AudioSampleDifference> for AudioSampleIndex {
    type Output = Self;

    fn add(self, rhs: AudioSampleDifference) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Mul<Channels> for AudioSampleDifference {
    type Output = usize;

    fn mul(self, rhs: Channels) -> Self::Output {
        self.0 as usize * rhs.0 as usize
    }
}

impl std::ops::Sub for AudioSampleIndex {
    type Output = AudioSampleDifference;

    fn sub(self, rhs: Self) -> Self::Output {
        AudioSampleDifference(self.0 - rhs.0)
    }
}

#[derive(PartialEq, Copy, Clone, derive_more::Add, derive_more::AddAssign)]
pub struct ModulationSampleIndex(pub u64);

#[derive(PartialEq, Copy, Clone, Add, Sub)]
pub struct ModulationSampleDifference(pub u64);

impl std::ops::Add<ModulationSampleDifference> for ModulationSampleIndex {
    type Output = Self;

    fn add(self, rhs: ModulationSampleDifference) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

#[derive(Copy, Clone)]
pub struct Channels(pub u16);

#[derive(Copy, Clone)]
pub struct SamplingRate(pub u32);

#[derive(Copy, Clone)]
pub struct ModulationRate(pub u32);

pub struct Engine {
    current_audio_sample: AudioSampleIndex,
    current_modulation_sample: ModulationSampleIndex,
    last_audio_sample_with_modulation: AudioSampleIndex,
    pub sampling_rate: SamplingRate,
    pub modulation_rate: ModulationRate,
    pub modulation_period: AudioSampleDifference,
    pub channels: Channels,
}

pub struct AudioTopology {
    components: AudioComponents,
    generated_components: usize,
    modulators: ModulationComponents,
    generated_modulators: usize,
}

impl AudioTopology {
    pub fn new() -> Self {
        Self {
            components: vec![],
            generated_components: 0,
            modulators: vec![],
            generated_modulators: 0,
        }
    }

    pub fn add_component<T: AudioComponent + Send + 'static>(
        &mut self,
        mut component: T,
    ) -> AudioComponentId {
        self.generated_components += 1;
        let id = AudioComponentId(NonZeroUsize::new(self.generated_components).unwrap());
        component.change_id(id);

        self.components.push(Box::new(component));

        id
    }

    pub fn add_modulator<T: ModulationComponent + Send + 'static>(
        &mut self,
        mut modulator: T,
    ) -> ModulatorId {
        self.generated_modulators += 1;
        let id = ModulatorId(NonZeroUsize::new(self.generated_modulators).unwrap());
        modulator.change_id(id);

        self.modulators.push(Box::new(modulator));

        id
    }
}

impl Engine {
    pub fn new(
        sampling_rate: SamplingRate,
        modulation_rate: ModulationRate,
        channels: Channels,
    ) -> Self {
        Self {
            current_audio_sample: AudioSampleIndex(0),
            current_modulation_sample: ModulationSampleIndex(0),
            last_audio_sample_with_modulation: AudioSampleIndex(0),
            sampling_rate,
            modulation_rate,
            modulation_period: AudioSampleDifference((sampling_rate.0 / modulation_rate.0) as u64),
            channels,
        }
    }

    pub fn advance(&mut self, topology: &mut AudioTopology, audio: &mut [f32]) {
        let total_samples = AudioSampleDifference((audio.len() / self.channels.0 as usize) as u64);
        let start_sample = self.current_audio_sample;
        assert_eq!(total_samples * self.channels, audio.len());

        if self.current_audio_sample == AudioSampleIndex(0) {
            self.process_modulation(topology);
        }

        let samples_before_next_modulation =
            self.last_audio_sample_with_modulation + self.modulation_period - start_sample;

        if samples_before_next_modulation > total_samples {
            self.process_audio(topology, audio, total_samples);
            return;
        }

        self.process_audio(
            topology,
            &mut audio[0..samples_before_next_modulation * self.channels],
            samples_before_next_modulation,
        );
        let mut samples_remaining = total_samples - samples_before_next_modulation;

        let mut current_sample_start_offset = samples_before_next_modulation;
        let audio_samples = self.modulation_period;

        while samples_remaining > AudioSampleDifference(0) {
            self.process_modulation(topology);
            let samples_to_process =
                samples_remaining.min(current_sample_start_offset + audio_samples);
            let current_sample_end_offset = current_sample_start_offset + samples_to_process;
            self.process_audio(
                topology,
                &mut audio[current_sample_start_offset * self.channels
                    ..current_sample_end_offset * self.channels],
                samples_to_process,
            );
            current_sample_start_offset = current_sample_end_offset;

            samples_remaining -= samples_to_process;
        }
    }

    fn process_modulation(&mut self, topology: &mut AudioTopology) {
        for m in &mut topology.modulators {
            m.process_modulation(self.current_modulation_sample);
        }

        for c in &mut topology.components {
            c.apply_modulations(topology.modulators.as_slice());
        }

        self.last_audio_sample_with_modulation = self.current_audio_sample;
        self.current_modulation_sample += ModulationSampleIndex(1);
    }

    fn process_audio(
        &mut self,
        topology: &mut AudioTopology,
        audio: &mut [f32],
        total_samples: AudioSampleDifference,
    ) {
        if total_samples.0 == 0 {
            return;
        }

        let start_sample = self.current_audio_sample;
        let end_sample = start_sample + total_samples;

        for c in &mut topology.components {
            c.process_audio(audio, start_sample..end_sample);
        }

        self.current_audio_sample = end_sample;
    }
}

pub fn empty_engine(
    sampling_rate: SamplingRate,
    modulation_rate: ModulationRate,
    channels: Channels,
) -> (Engine, AudioTopology) {
    let engine = Engine::new(sampling_rate, modulation_rate, channels);
    let audio_topology = AudioTopology::new();

    (engine, audio_topology)
}
