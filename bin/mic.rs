use weresocool::{
    examples::documentation,
    generation::{filename_to_render, RenderReturn, RenderType},
    portaudio::duplex_setup,
    ui::{get_args, no_file_name, were_so_cool_logo},
};


use error::Error;
use failure::Fail;

fn main() {
    match run() {
        Ok(_) => {}
        e => {
            for cause in Fail::iter_causes(&e.unwrap_err()) {
                println!("Failure caused by: {}", cause);
            }
        }
    }
}

fn run() -> Result<(), Error> {
    were_so_cool_logo();
    println!("{}", "       )))***=== MICROPHONE ===***(((  \n ");

    let args = get_args();

    if args.is_present("doc") {
        documentation();
    }

    let filename = args.value_of("filename");
    match filename {
        Some(_filename) => {}
        _ => no_file_name(),
    }

    let normal_form = match filename_to_render(filename.unwrap(), RenderType::NfBasisAndTable)? {
        RenderReturn::NfAndBasis(nf, _, _) => nf,
        _ => panic!("Error. Unable to generate NormalForm"),
    };

    println!("\nGenerating Composition ");
    let mut duplex_stream = duplex_setup(normal_form.operations)?;
    duplex_stream.start()?;

    while let true = duplex_stream.is_active()? {}

    duplex_stream.stop()?;
    Ok(())
}
