// main.rs
mod cli;
mod notebook;

use cli::get_args;
use notebook::process_notebook;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    let args = get_args();

    // Determine the input source
    let mut input: Box<dyn Read> = match args.input {
        Some(ref file_path) => {
            // Open the specified file
            Box::new(File::open(file_path).expect("Failed to open input file"))
        }
        None => {
            // Read from standard input
            Box::new(io::stdin())
        }
    };

    let mut content = String::new();
    input.read_to_string(&mut content).expect("Failed to read input");
    let output_text = process_notebook(content, args.format);
    match args.output {
        Some(ref file_path) => {
            // Write to the specified file
            let mut file = File::create(file_path).expect("Failed to create output file");
            file.write(output_text.as_bytes())
                .expect("Failed to write output");
        }
        None => {
            // Write to standard output
            // output_text
            println!("{}", output_text);
        }
    };
}
