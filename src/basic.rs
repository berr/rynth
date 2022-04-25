use std::num::NonZeroUsize;
use std::ops::Range;

#[derive(Copy, Clone)]
pub struct AudioComponentId(pub NonZeroUsize);

#[derive(Copy, Clone)]
pub struct ModulatorId(pub NonZeroUsize);

#[derive(Copy, Clone)]
pub struct ModulationId(pub NonZeroUsize);


pub trait AudioComponent {
    fn process_audio(&mut self, data: &mut [f32], sample_range: Range<u64>);
    fn id(&self) -> Option<AudioComponentId>;
    fn change_id(&mut self, new_id: AudioComponentId);
}

pub trait ModulationComponent {
    fn process_modulation(&mut self, sample: u64);
    fn id(&self) -> Option<ModulatorId>;
    fn change_id(&mut self, new_id: ModulatorId);
}


pub struct Modulation {
    id: ModulationId,
    level: f32,
    result: f32,
}

pub struct Parameter {
    value: f32,
    modulations: Vec<Modulation>,
    total_modulation: f32,
    final_value: f32,
}

impl Parameter {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            modulations: vec![],
            total_modulation: 0.0,
            final_value: value,
        }
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}