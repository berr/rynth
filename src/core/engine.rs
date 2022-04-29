use crate::core::concepts::{
    AudioSampleDifference, AudioSampleIndex, Channels, ModulationRate, ModulationSampleIndex,
    SamplingRate,
};
use crate::core::topology::AudioTopology;
use crate::core::{AudioComponentsStore, ModulationComponentsStore};

#[derive(Copy, Clone)]
pub struct EngineSpec {
    pub sampling_rate: SamplingRate,
    pub modulation_rate: ModulationRate,
    pub modulation_period: AudioSampleDifference,
    pub channels: Channels,
    pub max_samples_per_step: usize,
}

impl EngineSpec {
    pub fn new(
        sampling_rate: SamplingRate,
        modulation_rate: ModulationRate,
        channels: Channels,
        max_samples_per_step: usize,
    ) -> Self {
        Self {
            sampling_rate,
            modulation_rate,
            modulation_period: AudioSampleDifference((sampling_rate.0 / modulation_rate.0) as u64),
            channels,
            max_samples_per_step,
        }
    }
}

pub struct Engine {
    pub spec: EngineSpec,
    current_audio_sample: AudioSampleIndex,
    current_modulation_sample: ModulationSampleIndex,
    last_audio_sample_with_modulation: AudioSampleIndex,
}

impl Engine {
    pub fn new(spec: EngineSpec) -> Self {
        Self {
            spec,
            current_audio_sample: AudioSampleIndex(0),
            current_modulation_sample: ModulationSampleIndex(0),
            last_audio_sample_with_modulation: AudioSampleIndex(0),
        }
    }

    pub fn create_empty_topology(&self) -> AudioTopology {
        AudioTopology::new(self.spec)
    }

    pub fn advance(&mut self, topology: &mut AudioTopology, audio: &mut [f32]) {
        let total_samples =
            AudioSampleDifference((audio.len() / self.spec.channels.0 as usize) as u64);
        let start_sample = self.current_audio_sample;
        assert_eq!(total_samples * self.spec.channels, audio.len());

        let buffer = &mut topology.processing_buffer.as_mut_slice()[0..total_samples.0 as usize];

        if self.current_audio_sample == AudioSampleIndex(0) {
            self.process_modulation(
                &mut topology.modulation_components,
                &mut topology.audio_components,
            );
        }

        let samples_before_next_modulation =
            (self.last_audio_sample_with_modulation + self.spec.modulation_period) - start_sample;

        if samples_before_next_modulation > total_samples {
            self.process_audio(&mut topology.audio_components, buffer, total_samples);
            self.mix_output(buffer, audio);
            return;
        }

        self.process_audio(
            &mut topology.audio_components,
            &mut buffer[0..samples_before_next_modulation.0 as usize],
            samples_before_next_modulation,
        );
        let mut samples_remaining = total_samples - samples_before_next_modulation;

        let mut current_sample_start_offset = samples_before_next_modulation;
        let audio_samples = self.spec.modulation_period;

        while samples_remaining > AudioSampleDifference(0) {
            self.process_modulation(
                &mut topology.modulation_components,
                &mut topology.audio_components,
            );
            let samples_to_process = samples_remaining.min(audio_samples);
            let current_sample_end_offset = current_sample_start_offset + samples_to_process;
            self.process_audio(
                &mut topology.audio_components,
                &mut buffer
                    [current_sample_start_offset.0 as usize..current_sample_end_offset.0 as usize],
                samples_to_process,
            );
            current_sample_start_offset = current_sample_end_offset;

            samples_remaining -= samples_to_process;
        }

        self.mix_output(buffer, audio);
    }

    fn mix_output(&self, mono_output: &[f32], stereo_output: &mut [f32]) {
        let mut i = 0;
        for s in mono_output {
            for _ in 0..self.spec.channels.0 {
                stereo_output[i] = *s;
                i += 1;
            }
        }
    }

    fn process_modulation(
        &mut self,
        modulators: &mut ModulationComponentsStore,
        components: &mut AudioComponentsStore,
    ) {
        for m in modulators.iter_components_mut() {
            m.process_modulation(self.current_modulation_sample);
        }

        for c in components.iter_components_mut() {
            c.apply_modulations(modulators, self.current_audio_sample);
        }

        self.last_audio_sample_with_modulation = self.current_audio_sample;
        self.current_modulation_sample += ModulationSampleIndex(1);
    }

    fn process_audio(
        &mut self,
        components: &mut AudioComponentsStore,
        audio: &mut [f32],
        total_samples: AudioSampleDifference,
    ) {
        if total_samples.0 == 0 {
            return;
        }

        let start_sample = self.current_audio_sample;
        let end_sample = start_sample + total_samples;

        for c in components.iter_components_mut() {
            c.process_audio(audio, start_sample..end_sample);
        }

        self.current_audio_sample = end_sample;
    }
}

pub fn empty_engine(
    sampling_rate: SamplingRate,
    modulation_rate: ModulationRate,
    samples_per_step: usize,
    channels: Channels,
) -> (Engine, AudioTopology) {
    let spec = EngineSpec::new(sampling_rate, modulation_rate, channels, samples_per_step);
    let engine = Engine::new(spec);
    let audio_topology = engine.create_empty_topology();

    (engine, audio_topology)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{AlternatingModulator, ConstantGenerator};
    use crate::core::topology::AudioTopology;

    fn run_engine(engine: &mut Engine, topology: &mut AudioTopology, test_steps: usize) -> Vec<f32> {
        let channels = engine.spec.channels.0 as usize;
        let frame_size = engine.spec.max_samples_per_step;
        let mut obtained = vec![];

        let mut steps_so_far = 0;
        while steps_so_far < test_steps {
            let steps = frame_size.min(test_steps - steps_so_far);
            let samples = steps * channels;
            let mut buffer = vec![f32::NAN; samples];

            engine.advance(topology, buffer.as_mut_slice());
            obtained.extend(buffer.into_iter());

            steps_so_far += steps;
        }

        obtained
    }

    #[test]
    fn processes_audio() {
        let (mut engine, mut topology) = empty_engine(
            SamplingRate(48000),
            ModulationRate(100),
            128,
            Channels(1),
        );

        let mut generator = ConstantGenerator::default();
        generator.level.set_value(0.1);
        topology.add_component(generator);

        let test_samples = 48000;
        let obtained = run_engine(&mut engine, &mut topology, test_samples);

        assert_eq!(obtained, vec![0.1; test_samples]);
    }

    fn expected_alternating_modulation(audio_level: f32, modulation_amount: f32, samples: usize) -> Vec<f32> {
        (0..samples).map(|s|{
            let s2 = s / 480;
            if s2 % 2 == 0 {
                audio_level + modulation_amount
            } else {
                audio_level - modulation_amount
            }})
        .collect()
    }

    #[test]
    fn applies_modulation() {
        let (mut engine, mut topology) = empty_engine(
            SamplingRate(48000),
            ModulationRate(100),
            128,
            Channels(1),
        );

        let mut generator = ConstantGenerator::default();
        generator.level.set_value(0.1);

        let modulator_id = topology.add_modulator(AlternatingModulator::new(-1.0));
        generator.level.add_modulation(modulator_id, 0.5);

        topology.add_component(generator);

        let test_samples = 48000;
        let obtained = run_engine(&mut engine, &mut topology, test_samples);

        assert_eq!(obtained, expected_alternating_modulation(0.1, 0.5, test_samples));
    }

    #[test]
    fn applies_modulation_multiple_times_in_step() {
        let (mut engine, mut topology) = empty_engine(
            SamplingRate(48000),
            ModulationRate(100),
            1000,
            Channels(1),
        );

        let mut generator = ConstantGenerator::default();
        generator.level.set_value(0.1);

        let modulator_id = topology.add_modulator(AlternatingModulator::new(-1.0));
        generator.level.add_modulation(modulator_id, 0.5);

        topology.add_component(generator);

        let test_samples = 48000;
        let obtained = run_engine(&mut engine, &mut topology, test_samples);

        assert_eq!(obtained, expected_alternating_modulation(0.1, 0.5, test_samples));
    }

    #[test]
    fn applies_modulation_every_step() {
        let (mut engine, mut topology) = empty_engine(
            SamplingRate(48000),
            ModulationRate(100),
            480,
            Channels(1),
        );

        let mut generator = ConstantGenerator::default();
        generator.level.set_value(0.1);

        let modulator_id = topology.add_modulator(AlternatingModulator::new(-1.0));
        generator.level.add_modulation(modulator_id, 0.5);

        topology.add_component(generator);

        let test_samples = 48000;
        let obtained = run_engine(&mut engine, &mut topology, test_samples);

        assert_eq!(obtained, expected_alternating_modulation(0.1, 0.5, test_samples));
    }

    #[test]
    fn mixes_output() {
        let (mut engine, mut topology) = empty_engine(
            SamplingRate(48000),
            ModulationRate(100),
            480,
            Channels(2),
        );

        let mut generator = ConstantGenerator::default();
        generator.level.set_value(0.1);

        let modulator_id = topology.add_modulator(AlternatingModulator::new(-1.0));
        generator.level.add_modulation(modulator_id, 0.5);

        topology.add_component(generator);

        let test_samples = 48000;
        let obtained = run_engine(&mut engine, &mut topology, test_samples);

        let mut expected = vec![];
        for s in expected_alternating_modulation(0.1, 0.5, test_samples) {
            for _ in 0..engine.spec.channels.0 {
                expected.push(s);
            }
        }

        assert_eq!(obtained, expected);
    }
}