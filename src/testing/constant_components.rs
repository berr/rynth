use crate::core::concepts::{AudioSampleIndex, ModulationSampleIndex};
use crate::core::parameter::Parameter;
use crate::core::topology::ModulationComponentsStore;
use crate::core::traits::{AudioComponent, ModulationComponent};
use std::ops::Range;

pub struct ConstantGenerator {
    pub level: Parameter,
}

impl Default for ConstantGenerator {
    fn default() -> Self {
        Self {
            level: Parameter::new(1.0, -1.0, 1.0),
        }
    }
}

impl AudioComponent for ConstantGenerator {
    fn process_audio(&mut self, data: &mut [f32], _: Range<AudioSampleIndex>) {
        for sample in data.iter_mut() {
            *sample = self.level.final_value();
        }
    }

    fn apply_modulations(&mut self, modulators: &ModulationComponentsStore, _: AudioSampleIndex) {
        self.level.apply_modulations(modulators);
    }
}

pub struct AlternatingModulator {
    pub current_level: f32,
}

impl AlternatingModulator {
    pub fn new(level: f32) -> Self {
        Self {current_level: level}
    }
}

impl ModulationComponent for AlternatingModulator {

    fn process_modulation(&mut self, _sample: ModulationSampleIndex) {
        self.current_level = -self.current_level;
    }

    fn get_current_level(&self) -> f32 {
        self.current_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{AudioComponent, AudioSampleIndex};

    #[test]
    fn generates_constant_level() {
        let test_samples = 8000;
        let mut obtained = vec![0.0; test_samples];
        let expected_value = 0.6;

        let mut generator = ConstantGenerator::default();
        generator.level.set_value(expected_value);

        let range = AudioSampleIndex(0)..AudioSampleIndex((test_samples - 1) as u64);
        generator.process_audio(obtained.as_mut_slice(), range);

        assert_eq!(obtained, vec![expected_value; test_samples]);
    }

    #[test]
    fn applies_modulation_correctly() {
        let test_samples: u64 = 8000;

        let mut generator = ConstantGenerator::default();
        generator.level.set_value(0.5);

        let mut obtained = vec![f32::NAN; test_samples as usize];
        let range = AudioSampleIndex(0)..AudioSampleIndex(test_samples as u64);
        generator.process_audio(obtained.as_mut_slice(), range);

        assert_eq!(obtained, vec![0.5; test_samples as usize]);

        let mut modulators = ModulationComponentsStore::default();
        let constant_modulator_id = modulators.add_component(Box::new(AlternatingModulator::new(0.5)));

        generator.level.add_modulation(constant_modulator_id, 1.0);
        generator.apply_modulations(&modulators, AudioSampleIndex(8000));

        let mut obtained = vec![f32::NAN; test_samples as usize];
        let range = AudioSampleIndex(test_samples)..AudioSampleIndex(test_samples * 2);
        generator.process_audio(obtained.as_mut_slice(), range);

        assert_eq!(obtained, vec![1.0; test_samples as usize]);
    }
}
