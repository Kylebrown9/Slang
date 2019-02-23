use std::io::{ Result };

use crate::io_helpers::{ SimpleOutput };
use crate::tokenizer::{ Token };

use crate::trie::{ Trie, HasView };
use crate::trie::hash::{ HashTrie };

pub struct Macros {
    contents: HashTrie<PatternItem, Template>
}

#[derive(Hash, Eq, PartialEq, Clone)]
enum PatternItem {
    Var,

    Token {
        value: String
    },

    BlockVar {
        block_delim: BlockDelimiter
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
enum BlockDelimiter {
    SquareBracket,
    CurlyBracket,
    Parenthesis
}

type Template = Vec<TemplateItem>;

#[derive(Hash, Eq, PartialEq, Clone)]
enum TemplateItem {
    Text {
        data: String
    },
    Var {
        index: u8
    }
}

impl Macros {
    pub fn new() -> Self {
        Macros {
            contents: HashTrie::new()
        }
    }

    pub fn read_macros(&mut self, tokens: &[Token]) {
        //TODO implement
    }

    fn read_macro(&mut self, tokens: &[Token]) {
        //TODO implement
    }

    pub fn expand_tokens(&self, input: &[Token], out_stream: &mut SimpleOutput) -> Result<()> {
        //let trie_root = self.contents.as_view();

        let mut remaining = input;

        //TODO implement

        Ok(())
    }
}
