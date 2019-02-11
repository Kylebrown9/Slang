use std::io::{ Read, Write, Result };

use crate::builder::MacroDef;

pub fn expand_file(macro_def: MacroDef, in_stream: Box<Read>, out_stream: Box<Write>) -> Result<()> {
    Ok(())
}