use std::collections::{ HashMap };

use std::io::{ Result };

use crate::io_helpers::{ SimpleOutput };
use crate::tokenizer::{ Token };

use crate::trie::{ Trie, TrieView, TrieViewable };
use crate::trie::hash::{ HashTrie, HashTrieView };

pub struct Macros {
    contents: HashTrie<PatternItem, Vec<TemplateItem>>
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

    }

    fn read_macro(&mut self, tokens: &[Token]) {

    }

    pub fn expand_tokens(&self, input: &[Token], out_stream: &mut SimpleOutput) -> Result<()> {
        let trie_root = self.contents.as_view();

        let mut remaining = input;

        //TODO

        Ok(())
    }
}
