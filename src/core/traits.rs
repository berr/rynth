use crate::core::concepts::{
    AudioComponentId, AudioSampleIndex, ModulationSampleIndex, ModulatorId,
};
use crate::core::topology::DynModulationComponent;
use std::ops::Range;

pub trait AudioComponent {
    fn process_audio(&mut self, data: &mut [f32], sample_range: Range<AudioSampleIndex>);
    fn apply_modulations(
        &mut self,
        modulators: &[DynModulationComponent],
        sample: AudioSampleIndex,
    );

    fn id(&self) -> Option<AudioComponentId>;
    fn change_id(&mut self, new_id: AudioComponentId);
}

pub trait ModulationComponent {
    fn process_modulation(&mut self, sample: ModulationSampleIndex);
    fn get_current_level(&self) -> f32;
    fn id(&self) -> Option<ModulatorId>;
    fn change_id(&mut self, new_id: ModulatorId);
}
