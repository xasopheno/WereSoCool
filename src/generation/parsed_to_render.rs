use crate::{
    generation::{to_csv, to_json},
    ui::printed,
    write::{write_composition_to_mp3, write_composition_to_wav},
};

#[cfg(feature = "app")]
use pbr::ProgressBar;
#[cfg(feature = "app")]
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
#[cfg(feature = "app")]
use std::sync::{Arc, Mutex};
use weresocool_ast::{Defs, NormalForm, Term};
use weresocool_error::{Error, IdError};
use weresocool_instrument::renderable::{
    nf_to_vec_renderable, renderables_to_render_voices, RenderOp, Renderable,
};
use weresocool_instrument::{Basis, Oscillator, StereoWaveform};
use weresocool_parser::ParsedComposition;
use weresocool_shared::{default_settings, Settings};

const SETTINGS: Settings = default_settings();

#[derive(Clone, PartialEq, Debug)]
pub enum WavType {
    Wav { cli: bool },
    Mp3 { cli: bool },
}

#[derive(Clone, PartialEq, Debug)]
pub enum RenderType {
    Json4d,
    Csv1d,
    NfBasisAndTable,
    StereoWaveform,
    Wav(WavType),
    Stems,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
/// A stem is an audio file of a NormalForm which has been solo'd
/// and the audio rendered only contains operations with the name
/// in the NameSet.
pub struct Stem {
    /// Stem name
    pub name: String,
    /// Stem audio
    pub audio: Vec<u8>,
}

#[derive(Clone, PartialEq, Debug)]
#[allow(clippy::large_enum_variant)]
/// Target of a render
pub enum RenderReturn {
    Json4d(String),
    Csv1d(String),
    StereoWaveform(StereoWaveform),
    /// NormalForm, Basis, and Definition Table
    NfBasisAndTable(NormalForm, Basis, Defs),
    /// Wav or Mp3
    Wav(Vec<u8>),
    /// A vector of audio solo'd by names
    Stems(Vec<Stem>),
}

impl TryFrom<RenderReturn> for Vec<u8> {
    type Error = Error;
    fn try_from(r: RenderReturn) -> Result<Self, Self::Error> {
        match r {
            RenderReturn::Wav(audio) => Ok(audio),
            _ => Err(Error::with_msg(
                "Can only produce Vec<u8> from RenderReturn::Wav",
            )),
        }
    }
}

/// Generate a render of a parsed composition in the target render_type.
/// The filename is only used to for naming things when needed.
pub fn parsed_to_render(
    filename: &str,
    parsed_composition: ParsedComposition,
    return_type: RenderType,
) -> Result<RenderReturn, Error> {
    let parsed_main = parsed_composition.defs.terms.get("main");

    let nf = match parsed_main {
        Some(main) => match main {
            Term::Nf(nf) => nf,
            Term::Op(_) => {
                println!("main is not in Normal Form for some terrible reason.");
                return Err(Error::with_msg("Unrecoverable Error"));
            }
            Term::FunDef(_) => {
                println!("main as function not yet supported.");
                return Err(Error::with_msg("main as function not yet supported"));
            }
            Term::Lop(_) => {
                println!("main as list not yet supported.");
                return Err(Error::with_msg("main as list not yet supported"));
            }
            _ => unimplemented!(),
        },
        None => {
            return Err(IdError {
                id: "main".to_string(),
            }
            .into_error())
        }
    };

    let basis = Basis::from(parsed_composition.init);

    match return_type {
        RenderType::Stems => {
            let nf_names = nf.names();
            let names = parsed_composition.defs.stems.clone();
            if !names.is_subset(&nf_names) {
                let difference = names
                    .difference(&nf_names)
                    .into_iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(", ");

                return Err(Error::with_msg(format!(
                    "Stem names not found in composition: {}",
                    difference
                )));
            }

            if names.is_empty() {
                return Err(Error::with_msg("No stems to render"));
            }

            let mut result: Vec<Stem> = vec![];
            for name in names {
                let mut n = nf.clone();
                n.solo_ops_by_name(&name);
                let stereo_waveform = render(&basis, &n, &parsed_composition.defs)?;

                result.push(Stem {
                    name,
                    audio: write_composition_to_mp3(stereo_waveform)?,
                });
            }
            Ok(RenderReturn::Stems(result))
        }
        RenderType::NfBasisAndTable => Ok(RenderReturn::NfBasisAndTable(
            nf.clone(),
            basis,
            parsed_composition.defs,
        )),
        RenderType::Json4d => {
            to_json(
                &basis,
                nf,
                &parsed_composition.defs.clone(),
                filename.to_string(),
            )?;
            Ok(RenderReturn::Json4d("json".to_string()))
        }
        RenderType::Csv1d => {
            to_csv(
                &basis,
                nf,
                &parsed_composition.defs.clone(),
                filename.to_string(),
            )?;
            Ok(RenderReturn::Csv1d("json".to_string()))
        }
        RenderType::StereoWaveform => {
            let stereo_waveform = render(&basis, nf, &parsed_composition.defs)?;
            Ok(RenderReturn::StereoWaveform(stereo_waveform))
        }
        RenderType::Wav(wav_type) => match wav_type {
            WavType::Mp3 { cli } => {
                let stereo_waveform = render(&basis, nf, &parsed_composition.defs)?;
                let render_return = RenderReturn::Wav(write_composition_to_mp3(stereo_waveform)?);
                if cli {
                    let audio: Vec<u8> = Vec::try_from(render_return.clone())?;
                    let f = filename_to_renderpath(filename);
                    println!("filename: {}", &f);
                    write_audio_to_file(&audio, f.as_str(), "mp3")
                };
                Ok(render_return)
            }
            WavType::Wav { cli } => {
                let stereo_waveform = render(&basis, nf, &parsed_composition.defs)?;
                let render_return = RenderReturn::Wav(write_composition_to_wav(stereo_waveform)?);
                if cli {
                    let audio: Vec<u8> = Vec::try_from(render_return.clone())?;
                    let f = filename_to_renderpath(filename);
                    write_audio_to_file(&audio, f.as_str(), "wav")
                };
                Ok(render_return)
            }
        },
    }
}

fn filename_to_renderpath(filename: &str) -> String {
    let path = Path::new(filename).file_stem().unwrap();
    let f = format!("renders/{}", path.to_str().unwrap().to_string());
    f
}

pub fn write_audio_to_file(audio: &[u8], filename: &str, print_type: &str) {
    let mut file = File::create(format!("{}.{}", filename, print_type)).unwrap();
    file.write_all(audio).unwrap();
    printed(filename.to_string());
}

pub fn render(
    basis: &Basis,
    composition: &NormalForm,
    defs: &Defs,
) -> Result<StereoWaveform, Error> {
    let renderables = nf_to_vec_renderable(composition, defs, basis)?;
    let mut voices = renderables_to_render_voices(renderables);

    let mut result = StereoWaveform::new(0);
    loop {
        #[cfg(feature = "app")]
        let iter = voices.par_iter_mut();
        #[cfg(feature = "wasm")]
        let iter = voices.iter_mut();
        let batch: Vec<StereoWaveform> = iter
            .filter_map(|voice| voice.render_batch(SETTINGS.buffer_size, None))
            .collect();

        if !batch.is_empty() {
            let stereo_waveform = sum_all_waveforms(batch);
            result.append(stereo_waveform);
        } else {
            break;
        }
    }

    Ok(result)
}

#[cfg(feature = "app")]
fn create_pb_instance(n: usize) -> Arc<Mutex<ProgressBar<std::io::Stdout>>> {
    let mut pb = ProgressBar::new(n as u64);
    pb.format("╢w♬░╟");
    pb.message("Rendering:  ");
    Arc::new(Mutex::new(pb))
}

pub fn generate_waveforms(
    mut vec_sequences: Vec<Vec<RenderOp>>,
    show: bool,
) -> Vec<StereoWaveform> {
    if show {
        println!("Rendering {:?} waveforms", vec_sequences.len());
    }
    #[cfg(feature = "app")]
    let pb = create_pb_instance(vec_sequences.len());

    #[cfg(feature = "app")]
    let iter = vec_sequences.par_iter_mut();
    #[cfg(feature = "wasm")]
    let iter = vec_sequences.iter_mut();

    let vec_wav = iter
        .map(|ref mut vec_render_op: &mut Vec<RenderOp>| {
            #[cfg(feature = "app")]
            pb.lock().unwrap().add(1_u64);
            let mut osc = Oscillator::init(&default_settings());
            vec_render_op.render(&mut osc, None)
        })
        .collect();

    #[cfg(feature = "app")]
    pb.lock().unwrap().finish_print(&"".to_string());

    vec_wav
}

/// Sum a vec of StereoWaveform to a single stereo_waveform.
pub fn sum_all_waveforms(mut vec_wav: Vec<StereoWaveform>) -> StereoWaveform {
    // Sort the vectors by length
    sort_vecs(&mut vec_wav);

    // Get the length of the longest vector
    let max_len = vec_wav[0].l_buffer.len();

    let mut result = StereoWaveform::new(max_len);

    for wav in vec_wav {
        sum_vec(&mut result.l_buffer, &wav.l_buffer[..]);
        sum_vec(&mut result.r_buffer, &wav.r_buffer[..])
    }

    result
}

/// Sort a vec of StereoWaveform by length. Assumes both channels have the same
/// buffer length
fn sort_vecs(vec_wav: &mut Vec<StereoWaveform>) {
    vec_wav.sort_unstable_by(|a, b| b.l_buffer.len().cmp(&a.l_buffer.len()));
}

/// Sum two vectors. Assumes vector a is longer than or of the same length
/// as vector b.
pub fn sum_vec(a: &mut Vec<f64>, b: &[f64]) {
    for (ai, bi) in a.iter_mut().zip(b) {
        *ai += *bi;
    }
}
