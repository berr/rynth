use crate::basic::{AudioComponent, AudioComponentId, ModulationComponent, Parameter};
use crate::engine::{AudioSampleIndex, SamplingRate};
use std::ops::Range;

pub struct Oscillator {
    pub id: Option<AudioComponentId>,
    pub frequency: Parameter,
    pub level: Parameter,
    sampling_rate: SamplingRate,
}

impl Oscillator {
    pub fn new(frequency: f32, sampling_rate: SamplingRate) -> Self {
        Self {
            id: None,
            frequency: Parameter::new(frequency, 0.0, 20000.0),
            level: Parameter::new(frequency, 0.0, 1.0),
            sampling_rate,
        }
    }
}

impl AudioComponent for Oscillator {
    fn process_audio(&mut self, data: &mut [f32], sample_range: Range<AudioSampleIndex>) {
        let omega = 2.0 * std::f32::consts::PI * self.frequency.final_value();

        let range = sample_range.start.0..sample_range.end.0;

        for (frame, sample_index) in data.chunks_mut(2).zip(range) {
            let sample_index = AudioSampleIndex(sample_index);
            let t =
                (sample_index.0 % self.sampling_rate.0 as u64) as f32 / self.sampling_rate.0 as f32;
            let value = (t * omega).sin() * self.level.final_value();

            for sample_value in frame {
                *sample_value = value;
            }
        }
    }

    fn apply_modulations(&mut self, modulators: &[Box<dyn ModulationComponent + Send>]) {
        self.frequency.apply_modulations(modulators);
        self.level.apply_modulations(modulators);
    }

    fn id(&self) -> Option<AudioComponentId> {
        self.id
    }

    fn change_id(&mut self, new_id: AudioComponentId) {
        self.id = Some(new_id);
    }
}
