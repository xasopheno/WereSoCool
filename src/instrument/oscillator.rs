extern crate num_rational;
extern crate socool_ast;
use generation::parsed_to_render::r_to_f64;
use instrument::{stereo_waveform::StereoWaveform, voice::Voice};
use settings::Settings;
use socool_ast::operations::PointOp;
use std::f64::consts::PI;
fn tau() -> f64 {
    PI * 2.0
}

#[derive(Clone, Debug, PartialEq)]
pub struct Oscillator {
    pub voices: (Voice, Voice),
    pub portamento_length: usize,
    pub settings: Settings,
    pub sample_phase: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Origin {
    pub f: f64,
    pub p: f64,
    pub g: f64,
    pub l: f64,
}

fn point_op_to_gains(point_op: &PointOp, basis: &Origin) -> (f64, f64) {
    let pm = r_to_f64(point_op.pm);
    let pa = r_to_f64(point_op.pa);
    let g = r_to_f64(point_op.g);

    let l_gain = if *point_op.g.numer() == 0 {
        0.0
    } else {
        g * (((1.0 + pa * pm) + basis.p) / 2.0) * basis.g
    };

    let r_gain = if *point_op.g.numer() == 0 {
        0.0
    } else {
        g * (((-1.0 + pa * pm) + basis.p) / -2.0) * basis.g
    };

    (l_gain, r_gain)
}

impl Oscillator {
    pub fn init(settings: &Settings) -> Oscillator {
        Oscillator {
            voices: (Voice::init(0), Voice::init(1)),
            portamento_length: settings.buffer_size,
            settings: settings.clone(),
            sample_phase: 0.0,
        }
    }

    pub fn update(&mut self, basis: Origin, point_op: &PointOp, next_op: Option<PointOp>) {
        let fm = r_to_f64(point_op.fm);
        let fa = r_to_f64(point_op.fa);

        let (l_gain, r_gain) = point_op_to_gains(&point_op, &basis);
        let mut next_l_gain = 0.0;
        let mut next_r_gain = 0.0;
        let mut next_fm = 0.0;

        match next_op {
            Some(op) => {
                let (l, r) = point_op_to_gains(&op, &basis);
                next_l_gain = l;
                next_r_gain = r;
                next_fm = r_to_f64(op.fm);
            }
            None => {}
        }

        let (ref mut l_voice, ref mut r_voice) = self.voices;

        let silence_next_l = next_fm == 0.0 || next_l_gain == 0.0;
        let silence_next_r = next_fm == 0.0 || next_r_gain == 0.0;

        let decay_length = point_op.decay_length;

        l_voice.update(
            (basis.f * fm) + fa,
            l_gain,
            point_op.osc_type,
            silence_next_l,
            decay_length,
        );
        r_voice.update(
            (basis.f * fm) + fa,
            r_gain,
            point_op.osc_type,
            silence_next_r,
            decay_length,
        );
    }

    pub fn generate(&mut self, n_samples_to_generate: f64) -> StereoWaveform {
        let total_len = self.sample_phase + n_samples_to_generate;
        let length = total_len.floor() as usize;
        self.sample_phase = total_len.fract();
        let mut l_buffer: Vec<f64> = vec![0.0; length];
        let mut r_buffer: Vec<f64> = vec![0.0; length];
        let factor: f64 = tau() / self.settings.sample_rate;

        let (ref mut l_voice, ref mut r_voice) = self.voices;
        l_voice.generate_waveform(&mut l_buffer, self.portamento_length, factor);
        r_voice.generate_waveform(&mut r_buffer, self.portamento_length, factor);

        StereoWaveform { l_buffer, r_buffer }
    }
}
