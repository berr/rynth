use crate::core::component_store::{ComponentId, ComponentsStore};
use crate::core::traits::{AudioComponent, ModulationComponent};
use crate::core::EngineSpec;

pub type DynAudioComponent = Box<dyn AudioComponent>;
pub type AudioComponents = Vec<DynAudioComponent>;
pub type DynModulationComponent = Box<dyn ModulationComponent>;
pub type ModulationComponents = Vec<DynModulationComponent>;

#[derive(PartialEq, Copy, Clone)]
pub struct ModulationComponentIdTag;
pub type ModulationComponentId = ComponentId<ModulationComponentIdTag>;

#[derive(PartialEq, Copy, Clone)]
pub struct AudioComponentIdTag;
pub type AudioComponentId = ComponentId<AudioComponentIdTag>;

pub type AudioComponentsStore = ComponentsStore<dyn AudioComponent, AudioComponentIdTag>;
pub type ModulationComponentsStore =
    ComponentsStore<dyn ModulationComponent, ModulationComponentIdTag>;

pub struct AudioTopology {
    pub spec: EngineSpec,
    pub processing_buffer: Vec<f32>,
    pub audio_components: AudioComponentsStore,
    pub modulation_components: ModulationComponentsStore,
}

impl AudioTopology {
    pub fn new(spec: EngineSpec) -> Self {
        Self {
            spec,
            processing_buffer: vec![0.0; spec.max_samples_per_step],
            audio_components: ComponentsStore::default(),
            modulation_components: ComponentsStore::default(),
        }
    }

    pub fn add_component<T: 'static + AudioComponent>(&mut self, component: T) -> AudioComponentId {
        let boxed = Box::new(component);
        self.audio_components.add_component(boxed)
    }

    pub fn add_modulator<T: 'static + ModulationComponent>(
        &mut self,
        modulator: T,
    ) -> ModulationComponentId {
        let boxed = Box::new(modulator);
        self.modulation_components.add_component(boxed)
    }

    pub fn get_modulator(&self, id: ModulationComponentId) -> Option<&dyn ModulationComponent> {
        self.modulation_components.get_component(id)
    }
}
