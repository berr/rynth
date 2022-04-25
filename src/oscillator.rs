use std::ops::Range;
use crate::basic::{AudioComponent, AudioComponentId, Parameter};

pub struct Oscillator {
    pub id: Option<AudioComponentId>,
    pub frequency: Parameter,
    sample_rate: u32,
}

impl Oscillator {

    pub fn new(frequency: f32, sample_rate: u32) -> Self {
        Self {
            id: None,
            frequency: Parameter::new(frequency),
            sample_rate,
        }
    }
}

impl AudioComponent for Oscillator {

    fn process_audio(&mut self, data: &mut [f32], sample_range: Range<u64>) {
        // data.iter_mut().map(|v| *v = 0.0);
        // return;

        let omega = 2.0 * std::f32::consts::PI * self.frequency.value();

        for (frame, sample_index) in data.chunks_mut(2).zip(sample_range) {
            let t = sample_index as f32 / self.sample_rate as f32;
            let value = (t * omega).sin();

            for sample_value in frame {
                *sample_value = value;
            }
        }
    }

    fn id(&self) -> Option<AudioComponentId> {
        self.id
    }

    fn change_id(&mut self, new_id: AudioComponentId) {
        self.id = Some(new_id);
    }
}