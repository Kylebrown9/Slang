use std::collections::HashMap;

use std::io::{ Result };

use crate::io_helpers::{ SimpleInput, SimpleOutput };

pub struct Macros {
    contents: HashMap<String, MacroDef>
}

impl Macros {
    pub fn new() -> Self {
        Macros {
            contents: HashMap::new()
        }
    }

    pub fn expand(&self, in_stream: &mut SimpleInput, out_stream: &mut SimpleOutput) -> Result<()> {
        while let Some(line) = in_stream.next() {

            if let Some(prefix) = line.trim().split(" ").next() {

                if let Some(macro_def) = self.contents.get(prefix) {
                    macro_def.expand(line, in_stream, out_stream);
                    continue;
                }
            }

            out_stream.write(line);
        };

        Ok(())
    }
}

pub enum MacroDef {
    Simple {
        substitution: String
    },
    FuncStyle {
        items: Vec<FuncItems>
    },
    BlockStyle {
        items: Vec<BlockItems>
    }
}

enum FuncItems {
    Arg {
        index: u8
    },
    Text {
        data: String
    }
}

enum BlockItems {
    Block,
    Text {
        data: String
    }
}

impl MacroDef {
    fn expand(&self, first: String, in_stream: &mut SimpleInput, out_stream: &mut SimpleOutput) {
        match self {
            MacroDef::Simple { substitution } => {

            },
            MacroDef::FuncStyle  {items } => {

            },
            MacroDef::BlockStyle  {items } => {
                
            }
        }
    }
}