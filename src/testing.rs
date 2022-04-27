use crate::engine::{empty_engine, AudioTopology, Channels, Engine, ModulationRate, SamplingRate};
use crate::low_frequency_oscillator::LowFrequencyOscillator;
use crate::oscillator::Oscillator;

pub fn create_demo_engine() -> (Engine, AudioTopology) {
    let sampling_rate = SamplingRate(48000);
    let modulation_rate = ModulationRate(100);

    let (engine, mut topology) = empty_engine(sampling_rate, modulation_rate, Channels(2));

    let modulator_id = topology.add_modulator(LowFrequencyOscillator::new(2.0, modulation_rate));

    let mut oscillator = Oscillator::new(200.0, sampling_rate);
    oscillator.level.value = 0.4;
    oscillator.level.add_modulation(modulator_id, 0.5);

    // oscillator.frequency.add_modulation(modulator_id, 0.002);

    topology.add_component(oscillator);

    (engine, topology)
}