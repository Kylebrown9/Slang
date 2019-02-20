mod trie;
mod macro_def;
mod tokenizer;
use tokenizer::Tokenizer;

mod io_helpers;
use io_helpers::{ simplify_output, file_to_string, stdio_to_string };

mod builder;
use builder::build_macros;

use std::fs::File;
use std::io::{ Write, Result, Error, ErrorKind, stdout };

extern crate clap;
use clap::{ Arg, App };

fn main() {
    if let Err(error) = run_command() {
        println!("{}", error);
    }
}

fn run_command() -> Result<()> {
    let task = get_task()?;

    let tokenizer = Tokenizer::default();

    let macro_defs = build_macros(&tokenizer, task.macro_files)?;

    let input = match task.in_file {
        Some(in_file) => file_to_string(File::open(in_file)?),
        None => stdio_to_string()
    };

    let out_stream: Box<Write> = match task.out_file {
        Some(out_file) => Box::new(File::create(out_file)?),
        None => Box::new(stdout())
    };

    macro_defs.expand_tokens(
        &tokenizer.tokenize(&input), 
        &mut simplify_output(out_stream))
} 

/**
 * Represents a Slang macro expansion task
 * If in_file and out_file strings are not provided stdin and stdout
 * will be used instead
 */
struct Task {
    macro_files: Vec<String>,
    in_file: Option<String>,
    out_file: Option<String>
}

fn get_task() -> Result<Task> {
    let app = get_app();
    let matches = app.get_matches();

    Ok(Task {
        macro_files: matches
            .values_of("macrofiles")
            .ok_or(Error::new(ErrorKind::InvalidInput, "Must include at least one macrofile"))?
            .map(&str::to_string)
            .collect(),

        in_file: matches
            .value_of("infile")
            .map(&str::to_string),

        out_file: matches
            .value_of("outfile")
            .map(&str::to_string)
    })
}

fn get_app() -> App<'static, 'static> {
    App::new("Slang")
        .version("0.1.0")
        .author("Kyle Brown <kylebrw@gmail.com>")
        .about("A macro expansion program for simple language abstractions")
        .arg(Arg::with_name("macrofiles")
                .help("Macro definition files")
                .multiple(true)
                .takes_value(true)
                .min_values(1)
        )
        .arg(Arg::with_name("infile")
                .help("The input file to macro expand")
                .short("i")
                .long("input")
                .takes_value(true)
        )
        .arg(Arg::with_name("outfile")
                .help("The file to write the macro expanded input to")
                .short("o")
                .long("output")
                .takes_value(true)
        )
}