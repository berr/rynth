use crate::core::concepts::ModulatorId;
use crate::core::traits::ModulationComponent;

pub struct Modulation {
    modulator: ModulatorId,
    level: f32,
    result: f32,
}

pub struct Parameter {
    value: f32,
    minimum_value: f32,
    maximum_value: f32,
    modulations: Vec<Modulation>,
    total_modulation: f32,
    final_value: f32,
}

impl Parameter {
    pub fn new(value: f32, minimum_value: f32, maximum_value: f32) -> Self {
        assert!(minimum_value <= value && value <= maximum_value);

        Self {
            value,
            minimum_value,
            maximum_value,
            modulations: vec![],
            total_modulation: 0.0,
            final_value: value,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        assert!(self.minimum_value <= value && value <= self.maximum_value);
        self.value = value;
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn final_value(&self) -> f32 {
        self.final_value
    }

    pub fn add_modulation(&mut self, modulator: ModulatorId, level: f32) {
        let modulation = Modulation {
            modulator,
            level,
            result: 0.0,
        };

        self.modulations.push(modulation);
    }

    pub fn apply_modulations(&mut self, modulators: &[Box<dyn ModulationComponent + Send>]) {
        let min = self.minimum_value;
        let max = self.maximum_value;
        let map_modulation_domain = |m| (m + 1.0) / 2.0 * (max - min) + min;

        for modulator in modulators {
            self.modulations
                .iter_mut()
                .filter(|m| m.modulator == modulator.id().unwrap())
                .for_each(|modulation| {
                    modulation.result =
                        map_modulation_domain(modulator.get_current_level()) * modulation.level;
                });
        }

        self.total_modulation = self.modulations.iter().map(|m| m.result).sum();
        let raw_final_value = self.value + self.total_modulation;
        self.final_value = raw_final_value.clamp(min, max);
    }
}
