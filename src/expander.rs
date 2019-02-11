use std::io::{ Read, Write, Result, BufReader, BufRead, BufWriter };

use crate::macro_def::{ Macros };

pub fn expand_file(macro_def: Macros, in_stream: Box<Read>, out_stream: Box<Write>) -> Result<()> {
    let buffered_in = BufReader::new(in_stream);
    let buffered_out = BufWriter::new(out_stream);

    Ok(())
}