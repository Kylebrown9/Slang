use std::io::{ Read, Write, Result };

use crate::io_helpers::{ simplify_input, simplify_output };
use crate::macro_def::{ Macros };

pub fn expand_file(macro_def: Macros, in_stream: Box<Read>, out_stream: Box<Write>) -> Result<()> {
    macro_def.expand(
        &mut simplify_input(in_stream), 
        &mut simplify_output(out_stream)
    )?;

    Ok(())
}