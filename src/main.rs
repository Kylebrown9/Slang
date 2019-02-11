mod bufread_iter;
mod macro_def;

mod builder;
use builder::build_macros;

mod expander;
use expander::expand_file;

use std::fs::File;
use std::io::{ Read, Write, Result, Error, ErrorKind, stdin, stdout };

extern crate clap;
use clap::{ Arg, App };

fn main() {
    if let Err(error) = run_command() {
        println!("{}", error);
    }
}

fn run_command() -> Result<()> {
    let task = get_task()?;

    let macro_defs = build_macros(task.macro_files)?;

    let in_stream: Box<Read> = match task.in_file {
        Some(in_file) => Box::new(File::open(in_file)?),
        None => Box::new(stdin())
    };

    let out_stream: Box<Write> = match task.out_file {
        Some(out_file) => Box::new(File::create(out_file)?),
        None => Box::new(stdout())
    };

    expand_file(macro_defs, in_stream, out_stream)
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
                .long("--input")
                .takes_value(true)
        )
        .arg(Arg::with_name("outfile")
                .help("The file to write the macro expanded input to")
                .short("o")
                .long("--output")
                .takes_value(true)
        )
}