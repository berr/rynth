use crate::core::concepts::{AudioSampleIndex, SamplingRate};
use crate::core::parameter::Parameter;
use crate::core::traits::AudioComponent;
use crate::core::ModulationComponentsStore;
use std::ops::Range;

pub struct Oscillator {
    pub frequency: Parameter,
    pub phase_offset: f32,
    pub level: Parameter,
    sampling_rate: SamplingRate,
}

impl Oscillator {
    pub fn new(frequency: f32, sampling_rate: SamplingRate) -> Self {
        Self {
            frequency: Parameter::new(frequency, 0.0, 20000.0),
            phase_offset: 0.0,
            level: Parameter::new(1.0, 0.0, 1.0),
            sampling_rate,
        }
    }
}

impl AudioComponent for Oscillator {
    fn process_audio(&mut self, data: &mut [f32], sample_range: Range<AudioSampleIndex>) {
        let omega = 2.0 * std::f32::consts::PI * self.frequency.final_value();
        let cycle_length = self.sampling_rate.0 as f32 / self.frequency.final_value();

        let range = sample_range.start.0..sample_range.end.0;

        for (sample, sample_index) in data.iter_mut().zip(range) {
            let sample_index = AudioSampleIndex(sample_index);
            let t = (sample_index.0 as f32 % cycle_length) / self.sampling_rate.0 as f32;
            *sample = (t * omega + self.phase_offset).sin() * self.level.final_value();
        }
    }

    fn apply_modulations(
        &mut self,
        modulators: &ModulationComponentsStore,
        sample: AudioSampleIndex,
    ) {
        let old_cycle_length = self.sampling_rate.0 as f32 / self.frequency.final_value();
        let old_t = (sample.0 as f32 % old_cycle_length) / self.sampling_rate.0 as f32;
        let old_domain =
            2.0 * std::f32::consts::PI * self.frequency.final_value() * old_t + self.phase_offset;

        self.frequency.apply_modulations(modulators);

        let new_cycle_length = self.sampling_rate.0 as f32 / self.frequency.final_value();
        let new_t = (sample.0 as f32 % new_cycle_length) / self.sampling_rate.0 as f32;
        let new_domain = 2.0 * std::f32::consts::PI * self.frequency.final_value() * new_t;

        self.phase_offset = old_domain - new_domain;

        self.level.apply_modulations(modulators);
    }
}
