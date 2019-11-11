use crate::instrument::StereoWaveform;
use crate::settings::{default_settings, Settings};
use crate::write::write_output_buffer;
use error::Error;
use portaudio as pa;

pub fn output_setup(
    mut composition: StereoWaveform,
) -> Result<pa::Stream<pa::NonBlocking, pa::Output<f32>>, Error> {
    let pa = pa::PortAudio::new()?;
    let settings = default_settings();
    let output_settings = get_output_settings(&pa, &settings)?;
    let mut index = 0;
    let output_stream = pa.open_non_blocking_stream(output_settings, move |args| {
        let result = output_callback(args, settings.buffer_size, &mut composition, index);
        index += 1;
        result
    })?;

    Ok(output_stream)
}

fn output_callback(
    args: pa::OutputStreamCallbackArgs<'_, f32>,
    buffer_size: usize,
    composition: &mut StereoWaveform,
    index: usize,
) -> pa::stream::CallbackResult {
    let buffer_to_write = composition.get_buffer(index, buffer_size);
    match buffer_to_write {
        Some(result) => {
            write_output_buffer(args.buffer, result);
            pa::Continue
        }
        None => pa::Complete,
    }
}

pub fn get_output_settings(
    ref pa: &pa::PortAudio,
    ref settings: &Settings,
) -> Result<pa::stream::OutputSettings<f32>, Error> {
    let def_output = pa.default_output_device()?;
    let output_info = pa.device_info(def_output)?;
    // println!("Default output device info: {:#?}", &output_info);

    let latency = output_info.default_low_output_latency;
    let output_params =
        pa::StreamParameters::new(def_output, settings.channels, settings.interleaved, latency);

    let output_settings = pa::OutputStreamSettings::new(
        output_params,
        settings.sample_rate as f64,
        settings.buffer_size as u32,
    );

    Ok(output_settings)
}
