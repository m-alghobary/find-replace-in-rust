use std::env::{self};

fn main() {
    let args = env::args();
    let params = replace::Params::new(args);
    params.validate();

    println!("{:?}", params);
}
