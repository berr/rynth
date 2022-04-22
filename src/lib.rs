pub mod app;
pub mod audio_loop;
pub mod engine;
pub mod basic;
pub mod oscillator;
pub mod low_frequency_oscillator;


// pub struct Rynth {
//
//     pub sample_rate: usize,
//     pub channels: usize,
//     pub buffer_size: usize,
//
//     generator_app_state: OscillatorApplicationState,
//     generator_engine_state: OscillatorEngineState,
//     generator: Oscillator,
// }
//
//
// impl Rynth {
//
//     pub fn new(sample_rate: usize, channels: usize, buffer_size: usize) -> Self {
//         Rynth {
//             sample_rate,
//             channels,
//             buffer_size,
//             generator_app_state: OscillatorApplicationState{frequency: 440.0, sample_rate},
//             generator_engine_state: OscillatorEngineState{frequency: 440.0, sample_rate, current_time: 0},
//             generator: Oscillator{}}
//     }
//
//     pub fn process_audio(&mut self, buffer: &mut [f32]) {
//         self.generator.process_audio(&mut self.generator_engine_state, buffer);
//     }
// }
//
//
// struct OscillatorApplicationState {
//     pub frequency: f32,
//     pub sample_rate: usize,
// }
//
// struct OscillatorEngineState {
//     pub frequency: f32,
//     pub sample_rate: usize,
//     pub current_time: usize,
// }
//
// struct Oscillator {
// }
//
// impl Oscillator {
//
//     pub fn process_audio(&mut self, state: &mut OscillatorEngineState, data: &mut [f32]) {
//
//         for frame in data.chunks_mut(2) {
//
//             let envelope = state.current_time as f32 / state.sample_rate as f32;
//             let envelope = envelope.min(1.0);
//
//             let value = (state.current_time as f32 * state.frequency * 2.0 * std::f32::consts::PI / state.sample_rate as f32).sin() * 0.2;
//             let value = value * envelope;
//
//
//             for sample in frame {
//                 *sample = value;
//             }
//
//             state.current_time = (state.current_time + 1) % state.sample_rate;
//         }
//     }
// }
//
