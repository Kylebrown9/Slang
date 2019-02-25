use std::io::{ Result };
use std::collections::{ LinkedList };

use crate::io_helpers::{ SimpleOutput };
use crate::tokenizer::{ Token };

use crate::trie::hash::{ HashTrie };

/// The data type representing accumulated macros
/// It associates a sequence of pattern items with a template
pub struct Macros {
    contents: HashTrie<PatternItem, Template>
}

/// Pattern Items dictate what structure matching inputs
/// must obey as well as how to read in values from
/// matching values
#[derive(Hash, Eq, PartialEq, Clone)]
enum PatternItem {
    /// Constrains matches to include this exact token value
    MatchToken {
        value: String
    },

    /// Reads in a variable from exactly one token
    TokenVar,

    /// Constrains matches to include an occurance of a previously
    /// parsed in token variable
    MatchTokenVar {
        index: u8
    },

    /// Reads in a variable from one or more tokens
    SequenceVar,

    /// Constrains matches to include the specified block type
    /// and applies the inner pattern to the token sequence 
    BlockPattern {
        block_delim: BlockDelimiter,
        inner_pattern: Vec<PatternItem>
    }
}

/// Identifies the three block types which are
/// pair matched and parsed into blocks
#[derive(Hash, Eq, PartialEq, Clone)]
enum BlockDelimiter {
    SquareBracket,
    CurlyBracket,
    Parenthesis
}

type Template = Vec<TemplateItem>;

/// A template item represents a single value to display
/// when a template is rendered
#[derive(Hash, Eq, PartialEq, Clone)]
enum TemplateItem {
    /// Represents a sequence of character values to reproduce
    /// exactly in rendered templates. May be of an arbitrary length
    /// and contain arbitrary singleton and separator characters
    Text {
        data: String
    },

    /// Represents a parsed in variable to displayed exactly as it was
    /// read in by the corresponding pattern
    Var {
        index: u8
    }
}

impl Macros {
    /// Constructs an empty Macros instance for holding macro definitions
    pub fn new() -> Self {
        Macros {
            contents: HashTrie::new()
        }
    }

    /// Reads in macros from a token slice
    pub fn read_macros(&mut self, tokens: &[Token]) {
        //TODO implement
    }

    /// Reads in a single macro from a token slice
    fn read_macro(&mut self, tokens: &[Token]) {
        //TODO implement
    }

    /// Performs macro expansion on a slice of tokens and outputs
    /// the expanded values using the out_stream
    pub fn expand_tokens(&self, input: &[Token], out_stream: &mut SimpleOutput) -> Result<()> {
        //let trie_root = self.contents.as_view();

        let mut remaining = input;

        let mut variable_buffer: LinkedList<&[Token]> = LinkedList::new();

        while !remaining.is_empty() {
            let token = &input[0];

            match token.value {
                "{" => {

                },

                "[" => {

                },

                "(" => {

                },

                _ => {

                }
            }

        }

        //TODO implement

        Ok(())
    }
}

struct BlockParse<'a> {
    block_tokens: &'a Token<'a>,
    remaining: &'a Token<'a>
}

fn parse_block<'a>(delim: BlockDelimiter, input: &'a [Token<'a>]) -> BlockParse<'a> {
    let curly_level =   if delim == BlockDelimiter::CurlyBracket { 1 } else { 0 };
    let square_level =  if delim == BlockDelimiter::SquareBracket { 1 } else { 0 };
    let paren_level =   if delim == BlockDelimiter::Parenthesis { 1 } else { 0 };

    // TODO implement

    BlockParse {
        block_tokens: 1,
        remaining: 
    }
}