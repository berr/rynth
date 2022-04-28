use crate::core::concepts::{AudioComponentId, ModulatorId};
use crate::core::traits::{AudioComponent, ModulationComponent};
use std::num::NonZeroUsize;

pub type AudioComponents = Vec<Box<dyn AudioComponent + Send>>;
pub type ModulationComponents = Vec<Box<dyn ModulationComponent + Send>>;

pub struct AudioTopology {
    pub components: AudioComponents,
    pub modulators: ModulationComponents,
    generated_components: usize,
    generated_modulators: usize,
}

impl Default for AudioTopology {
    fn default() -> Self {
        Self {
            components: vec![],
            generated_components: 0,
            modulators: vec![],
            generated_modulators: 0,
        }
    }
}

impl AudioTopology {

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
