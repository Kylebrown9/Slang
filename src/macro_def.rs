use std::collections::{ HashMap, HashSet };

use std::io::{ Result };

use crate::io_helpers::{ SimpleInput, SimpleOutput };
use crate::tokenizer::{ Tokenizer, Token };

pub struct Macros {
    contents: HashMap<String, MacroDef>
}

impl Macros {
    pub fn new() -> Self {
        Macros {
            contents: HashMap::new()
        }
    }

    pub fn add_macro(&mut self, token: String, macro_def: MacroDef) {
        self.contents.insert(token, macro_def);
    }

    pub fn expand(&self, input: String, out_stream: &mut SimpleOutput) -> Result<()> {
        let tokenizer = Tokenizer::default();

        let tokens: Vec<Token> = tokenizer.tokenize(&input);

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
    fn expand(&self, tokens: &[Token], out_stream: &mut SimpleOutput) {
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