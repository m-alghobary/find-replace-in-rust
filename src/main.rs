use std::env::{self};

fn main() -> std::io::Result<()> {
    let args = env::args();
    let params = replace::Params::new(args);
    params.validate();

    let in_file = std::fs::read_to_string(&params.input_file)?;

    let mut result = Vec::<String>::new();
    for line in in_file.lines() {
        if line.contains(&params.term) {
            result.push(line.replace(&params.term, "foo"));
        }
    }

    let result = result.join("\n");

    println!("{}", result);

    std::fs::write(&params.input_file, result)?;

    println!("Replace done");

    Ok(())
}
