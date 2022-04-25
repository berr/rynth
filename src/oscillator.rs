use crate::basic::{AudioComponent, AudioComponentId, ModulationComponent, Parameter};
use std::ops::Range;

pub struct Oscillator {
    pub id: Option<AudioComponentId>,
    pub frequency: Parameter,
    pub level: Parameter,
    sample_rate: u32,
}

impl Oscillator {
    pub fn new(frequency: f32, sample_rate: u32) -> Self {
        Self {
            id: None,
            frequency: Parameter::new(frequency, 0.0, 20000.0),
            level: Parameter::new(frequency, 0.0, 1.0),
            sample_rate,
        }
    }
}

impl AudioComponent for Oscillator {
    fn process_audio(&mut self, data: &mut [f32], sample_range: Range<u64>) {
        let omega = 2.0 * std::f32::consts::PI * self.frequency.final_value();

        for (frame, sample_index) in data.chunks_mut(2).zip(sample_range) {
            let t = sample_index as f32 / self.sample_rate as f32;
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
