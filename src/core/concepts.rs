use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(PartialEq, Copy, Clone)]
pub struct AudioSampleIndex(pub u64);

#[derive(Copy, Clone, Add, AddAssign, Sub, SubAssign, Ord, PartialOrd, Eq, PartialEq)]
pub struct AudioSampleDifference(pub u64);

impl std::ops::Add<AudioSampleDifference> for AudioSampleIndex {
    type Output = Self;

    fn add(self, rhs: AudioSampleDifference) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Mul<Channels> for AudioSampleDifference {
    type Output = usize;

    fn mul(self, rhs: Channels) -> Self::Output {
        self.0 as usize * rhs.0 as usize
    }
}

impl std::ops::Sub for AudioSampleIndex {
    type Output = AudioSampleDifference;

    fn sub(self, rhs: Self) -> Self::Output {
        AudioSampleDifference(self.0 - rhs.0)
    }
}

#[derive(PartialEq, Copy, Clone, derive_more::Add, derive_more::AddAssign)]
pub struct ModulationSampleIndex(pub u64);

#[derive(PartialEq, Copy, Clone, Add, Sub)]
pub struct ModulationSampleDifference(pub u64);

impl std::ops::Add<ModulationSampleDifference> for ModulationSampleIndex {
    type Output = Self;

    fn add(self, rhs: ModulationSampleDifference) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

#[derive(Copy, Clone)]
pub struct Channels(pub u16);

#[derive(Copy, Clone)]
pub struct SamplingRate(pub u32);

#[derive(Copy, Clone)]
pub struct ModulationRate(pub u32);

// #[derive(Copy, Clone)]
// pub struct AudioComponentId(pub NonZeroUsize);
//
// #[derive(Copy, Clone, PartialEq)]
// pub struct ModulatorId(pub NonZeroUsize);
