use std::io::{ Read, Write, Result, BufReader, BufRead, BufWriter };

use crate::bufread_iter::BufReadIter;
use crate::macro_def::{ Macros };

pub fn expand_file(macro_def: Macros, in_stream: Box<Read>, out_stream: Box<Write>) -> Result<()> {
    let mut buffered_in = BufReader::new(in_stream);
    let mut buffered_out = BufWriter::new(out_stream);

    let mut line_iter = BufReadIter::from(buffered_in);

    while let Some(line) = line_iter.next() {
        
    }

    Ok(())
}