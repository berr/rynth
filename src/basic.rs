use std::ops::Range;

pub trait AudioComponent {
    fn process_audio(&mut self, data: &mut [f32], sample_range: Range<u64>);
}

pub struct ModulationId(usize);

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