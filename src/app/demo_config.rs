use crate::components::{LowFrequencyOscillator, Oscillator};
use crate::core::{empty_engine, AudioTopology, Channels, Engine, ModulationRate, SamplingRate};

pub fn create_demo_engine() -> (Engine, AudioTopology) {
    let sampling_rate = SamplingRate(48000);
    let modulation_rate = ModulationRate(100);
    let samples_per_step = 128;

    let (engine, mut topology) = empty_engine(
        sampling_rate,
        modulation_rate,
        samples_per_step,
        Channels(2),
    );

    let modulator1_id = topology.add_modulator(LowFrequencyOscillator::new(2.0, modulation_rate));
    let modulator2_id = topology.add_modulator(LowFrequencyOscillator::new(10.0, modulation_rate));

    let mut oscillator = Oscillator::new(500.0, sampling_rate);

    oscillator.level.set_value(0.3);
    oscillator.level.add_modulation(modulator1_id, 0.2);
    oscillator.frequency.add_modulation(modulator2_id, 0.01); // 200Hz range

    topology.add_component(oscillator);

    (engine, topology)
}
