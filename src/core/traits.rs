use crate::core::concepts::{AudioSampleIndex, ModulationSampleIndex};
use crate::core::ModulationComponentsStore;
use std::ops::Range;

pub trait AudioComponent: Send {
    fn process_audio(&mut self, data: &mut [f32], sample_range: Range<AudioSampleIndex>);
    fn apply_modulations(
        &mut self,
        modulators: &ModulationComponentsStore,
        sample: AudioSampleIndex,
    );
}

pub trait ModulationComponent: Send {
    fn process_modulation(&mut self, sample: ModulationSampleIndex);
    fn get_current_level(&self) -> f32;
}
