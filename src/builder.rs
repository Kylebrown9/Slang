use std::io::{ Result, Read };
use std::fs::File;

use crate::tokenizer::Tokenizer;
use crate::macro_def::{ Macros };

pub fn build_macros(tokenizer: &Tokenizer, macro_files: Vec<String>) -> Result<Macros> {
    let mut macros = Macros::new();

    for file_name in macro_files {
        read_macros(&mut macros, tokenizer, file_name)?;
    }

    Ok(macros)
}

fn read_macros(macros: &mut Macros, tokenizer: &Tokenizer, file_name: String) -> Result<()>{
    let mut file = File::open(file_name)?;

    let mut file_data = String::new();

    file.read_to_string(&mut file_data);

    let tokens = tokenizer.tokenize(&file_data);

    macros.read_macros(&tokens);

    Ok(())
}