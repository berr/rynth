use crate::core::concepts::{ModulationRate, ModulationSampleIndex, ModulatorId};
use crate::core::parameter::Parameter;
use crate::core::traits::ModulationComponent;

pub struct LowFrequencyOscillator {
    pub id: Option<ModulatorId>,
    pub frequency: Parameter,
    pub current_level: f32,
    sample_rate: ModulationRate,
}

impl LowFrequencyOscillator {
    pub fn new(frequency: f32, sample_rate: ModulationRate) -> Self {
        Self {
            id: None,
            frequency: Parameter::new(frequency, 0.0, 300.0),
            sample_rate,
            current_level: 0.0,
        }
    }
}

impl ModulationComponent for LowFrequencyOscillator {
    fn process_modulation(&mut self, sample: ModulationSampleIndex) {
        let omega = 2.0 * std::f32::consts::PI * self.frequency.get_value();
        let t = (sample.0 % self.sample_rate.0 as u64) as f32 / self.sample_rate.0 as f32;
        self.current_level = (t * omega).sin();
    }

    fn get_current_level(&self) -> f32 {
        self.current_level
    }

    fn id(&self) -> Option<ModulatorId> {
        self.id
    }

    fn change_id(&mut self, new_id: ModulatorId) {
        self.id = Some(new_id);
    }
}
