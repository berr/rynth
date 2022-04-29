use crate::core::concepts::{
    AudioSampleDifference, AudioSampleIndex, Channels, ModulationRate, ModulationSampleIndex,
    SamplingRate,
};
use crate::core::topology::{AudioTopology, DynAudioComponent, DynModulationComponent};

pub struct Engine {
    pub sampling_rate: SamplingRate,
    pub modulation_rate: ModulationRate,
    pub modulation_period: AudioSampleDifference,
    pub channels: Channels,
    pub max_samples_per_step: usize,

    current_audio_sample: AudioSampleIndex,
    current_modulation_sample: ModulationSampleIndex,
    last_audio_sample_with_modulation: AudioSampleIndex,
}

impl Engine {
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

            current_audio_sample: AudioSampleIndex(0),
            current_modulation_sample: ModulationSampleIndex(0),
            last_audio_sample_with_modulation: AudioSampleIndex(0),
        }
    }

    pub fn advance(&mut self, topology: &mut AudioTopology, audio: &mut [f32]) {
        let total_samples = AudioSampleDifference((audio.len() / self.channels.0 as usize) as u64);
        let start_sample = self.current_audio_sample;
        assert_eq!(total_samples * self.channels, audio.len());

        let buffer = &mut topology.processing_buffer.as_mut_slice()[0..total_samples.0 as usize];

        if self.current_audio_sample == AudioSampleIndex(0) {
            self.process_modulation(
                topology.modulators.as_mut_slice(),
                topology.components.as_mut_slice(),
            );
        }

        let samples_before_next_modulation =
            self.last_audio_sample_with_modulation + self.modulation_period - start_sample;

        if samples_before_next_modulation > total_samples {
            self.process_audio(&mut topology.components, buffer, total_samples);
            self.mix_output(buffer, audio);
            return;
        }

        self.process_audio(
            topology.components.as_mut_slice(),
            &mut buffer[0..samples_before_next_modulation.0 as usize],
            samples_before_next_modulation,
        );
        let mut samples_remaining = total_samples - samples_before_next_modulation;

        let mut current_sample_start_offset = samples_before_next_modulation;
        let audio_samples = self.modulation_period;

        while samples_remaining > AudioSampleDifference(0) {
            self.process_modulation(
                topology.modulators.as_mut_slice(),
                topology.components.as_mut_slice(),
            );
            let samples_to_process =
                samples_remaining.min(current_sample_start_offset + audio_samples);
            let current_sample_end_offset = current_sample_start_offset + samples_to_process;
            self.process_audio(
                topology.components.as_mut_slice(),
                &mut buffer[current_sample_start_offset.0 as usize
                    ..current_sample_end_offset.0 as usize],
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
            for _ in 0..self.channels.0 {
                stereo_output[i] = *s;
                i += 1;
            }
        }
    }

    fn process_modulation(
        &mut self,
        modulators: &mut [DynModulationComponent],
        components: &mut [DynAudioComponent],
    ) {
        for m in modulators.iter_mut() {
            m.process_modulation(self.current_modulation_sample);
        }

        for c in components.iter_mut() {
            c.apply_modulations(&modulators, self.current_audio_sample);
        }

        self.last_audio_sample_with_modulation = self.current_audio_sample;
        self.current_modulation_sample += ModulationSampleIndex(1);
    }

    fn process_audio(
        &mut self,
        components: &mut [DynAudioComponent],
        audio: &mut [f32],
        total_samples: AudioSampleDifference,
    ) {
        if total_samples.0 == 0 {
            return;
        }

        let start_sample = self.current_audio_sample;
        let end_sample = start_sample + total_samples;

        for c in components {
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
    let engine = Engine::new(sampling_rate, modulation_rate, channels, samples_per_step);
    let audio_topology = AudioTopology::new(samples_per_step);

    (engine, audio_topology)
}
