use crate::basic::{ModulationComponent, Parameter, ModulatorId};

pub struct LowFrequencyOscillator {
    pub id: Option<ModulatorId>,
    pub frequency: Parameter,
    pub current_level: f32,
    sample_rate: u32,
}

impl LowFrequencyOscillator {

    pub fn new(frequency: f32, sample_rate: u32) -> Self {
        Self {
            id: None,
            frequency: Parameter::new(frequency),
            sample_rate,
            current_level: 0.0,
        }
    }
}

impl ModulationComponent for LowFrequencyOscillator {

    fn process_modulation(&mut self, sample: u64) {
        let omega = 2.0 * std::f32::consts::PI * self.frequency.value();
        let t = sample as f32 / self.sample_rate as f32;
        self.current_level = (t * omega).sin();
    }

    fn id(&self) -> Option<ModulatorId> {
        self.id
    }

    fn change_id(&mut self, new_id: ModulatorId) {
        self.id = Some(new_id);
    }
}