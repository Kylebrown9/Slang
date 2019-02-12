use std::collections::{ HashMap };

use std::io::{ Result };

use crate::io_helpers::{ SimpleOutput };
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

        let mut remaining = &tokens[..];

        while !remaining.is_empty() {
            let current_token = &remaining[0];

            if let Some(macro_def) = self.contents.get(current_token.value) {
                remaining = macro_def.expand(remaining, out_stream);
            } else {
                out_stream.write(current_token.value);
                out_stream.write(current_token.suffix);
            }
        }

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
    fn expand<'a>(&self, tokens: &'a [Token<'a>], out_stream: &mut SimpleOutput) -> &'a [Token<'a>] {
        match self {
            MacroDef::Simple { substitution } => {
                
            },
            MacroDef::FuncStyle  {items } => {

            },
            MacroDef::BlockStyle  {items } => {
                
            }
        }

        tokens //TODO make represent actual remaining
    }
}