mod asr;
mod frequency;
mod gain;
mod loudness;
pub mod oscillator;
mod sample;
pub mod stereo_waveform;
pub mod voice;

#[cfg(test)]
mod asr_test;
#[cfg(test)]
mod test;

pub use self::{
    oscillator::{Basis, Oscillator},
    stereo_waveform::{Normalize, StereoWaveform},
};
