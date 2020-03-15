use weresocool::{
    examples::documentation,
    generation::parsed_to_render::{r_to_f64, RenderReturn, RenderType},
    interpretable::{InputType::Filename, Interpretable},
    portaudio::duplex_setup,
    renderable::nf_to_vec_renderable,
    ui::{get_args, no_file_name, were_so_cool_logo},
};

use failure::Fail;
use weresocool_error::Error;

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
    println!("       )))***=== REAL<GOOD>TIME ===***(((  \n ");

    let args = get_args();

    if args.is_present("doc") {
        documentation();
    }

    let filename = args.value_of("filename");
    match filename {
        Some(_filename) => {}
        _ => no_file_name(),
    }

    let (nf, basis, table) = match Filename(filename.unwrap()).make(RenderType::NfBasisAndTable)? {
        RenderReturn::NfBasisAndTable(nf, basis, table) => (nf, basis, table),
        _ => panic!("Error. Unable to generate NormalForm"),
    };
    let renderables = nf_to_vec_renderable(&nf, &table, &basis);

    println!("\nGenerating Composition ");
    let mut duplex_stream = duplex_setup(r_to_f64(basis.f), renderables)?;
    duplex_stream.start()?;

    while let true = duplex_stream.is_active()? {}

    duplex_stream.stop()?;
    Ok(())
}
