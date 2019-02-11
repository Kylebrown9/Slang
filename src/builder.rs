use std::io::Result;

use crate::macro_def::{ Macros, MacroDef };

pub fn build_macros(macro_files: Vec<String>) -> Result<Macros> {
    Ok(Macros::new())
}