use std::io::Result;

use crate::macro_def::{ Macros, MacroDef };

pub fn build_macros(macro_files: Vec<String>) -> Result<Macros> {
    let mut macros = Macros::new();

    for file_name in macro_files {
        read_macros(&mut macros, file_name)?;
    }

    Ok(macros)
}

fn read_macros(macros: &mut Macros, file_name: String) -> Result<()>{
    

    Ok(())
}