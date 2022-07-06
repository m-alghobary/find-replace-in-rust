use std::{env::Args, process};

#[derive(Debug)]
pub struct Params {
    pub search: String,
    pub replace: String,
    pub input_file: String,
    pub output_file: String,
}

impl Params {
    fn default() -> Self {
        Params {
            search: String::new(),
            replace: String::new(),
            input_file: String::new(),
            output_file: String::new(),
        }
    }

    pub fn new(mut args: Args) -> Self {
        // forget about the first arg (its the path to our program)
        args.next();

        let args: Vec<String> = args.collect();
        let mut result = Params::default();

        for (i, arg) in args.iter().enumerate() {
            if arg == "-t" {
                result.search = args.get(i + 1).unwrap_or(&String::new()).to_string();
            }

            if arg == "-r" {
                result.replace = args.get(i + 1).unwrap_or(&String::new()).to_string();
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
        if self.search == "" {
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

pub fn replace(in_file: String, search: &str, replace: &str) -> String {
    let mut result = Vec::<String>::new();

    for line in in_file.lines() {
        let mut line: String = line.to_string();

        if line.contains(search) {
            line = line.replace(search, replace);
        }

        result.push(line);
    }

    let result = result.join("\n");
    result
}
