use std::env::{self};

fn main() -> std::io::Result<()> {
    let args = env::args();
    let mut params = replace::Params::new(args);
    params.validate();

    let in_file = std::fs::read_to_string(&params.input_file)?;

    let result = replace::replace(in_file, &params.search, &params.replace);

    std::fs::write(&params.output_file, result)?;

    println!("Replace done");

    Ok(())
}
