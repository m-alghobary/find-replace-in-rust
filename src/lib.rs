use std::{env::Args, process};

#[derive(Debug)]
pub struct Params {
    pub term: String,
    pub input_file: String,
    pub output_file: String,
}

impl Params {
    pub fn new(mut args: Args) -> Self {
        args.next();
        let args: Vec<String> = args.collect();

        let mut result = Params {
            term: String::new(),
            input_file: String::new(),
            output_file: String::new(),
        };

        for (i, arg) in args.iter().enumerate() {
            if arg == "-t" {
                result.term = args.get(i + 1).unwrap_or(&String::new()).to_string();
            }

            if arg == "-i" {
                result.input_file = args.get(i + 1).unwrap_or(&String::new()).to_string();
            }

            if arg == "-o" {
                result.output_file = args.get(i + 1).unwrap_or(&String::new()).to_string();
            }
        }

        result
    }

    pub fn validate(&mut self) {
        if self.term == "" {
            println!("Please add a search term!");
            process::exit(1);
        }

        if self.input_file == "" {
            println!("Please pass a file to use!");
            process::exit(1);
        }

        if self.output_file == "" {
            self.output_file = self.input_file.to_owned();
        }
    }
}
