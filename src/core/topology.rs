use crate::core::concepts::{AudioComponentId, ModulatorId};
use crate::core::traits::{AudioComponent, ModulationComponent};
use std::num::NonZeroUsize;

pub type DynAudioComponent = Box<dyn AudioComponent + Send>;
pub type AudioComponents = Vec<DynAudioComponent>;
pub type DynModulationComponent = Box<dyn ModulationComponent + Send>;
pub type ModulationComponents = Vec<DynModulationComponent>;

pub struct AudioTopology {
    pub processing_buffer: Vec<f32>,
    pub components: AudioComponents,
    generated_components: usize,
    pub modulators: ModulationComponents,
    generated_modulators: usize,
}

impl AudioTopology {
    pub fn new(max_samples_per_step: usize) -> Self {
        Self {
            processing_buffer: vec![0.0; max_samples_per_step],
            components: vec![],
            generated_components: 0,
            modulators: vec![],
            generated_modulators: 0,
        }
    }

    pub fn add_component<T: AudioComponent + Send + 'static>(
        &mut self,
        mut component: T,
    ) -> AudioComponentId {
        self.generated_components += 1;
        let id = AudioComponentId(NonZeroUsize::new(self.generated_components).unwrap());
        component.change_id(id);

        self.components.push(Box::new(component));

        id
    }

    pub fn add_modulator<T: ModulationComponent + Send + 'static>(
        &mut self,
        mut modulator: T,
    ) -> ModulatorId {
        self.generated_modulators += 1;
        let id = ModulatorId(NonZeroUsize::new(self.generated_modulators).unwrap());
        modulator.change_id(id);

        self.modulators.push(Box::new(modulator));

        id
    }
}
