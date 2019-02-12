use std::collections::{ HashMap, HashSet };

pub struct Tokenizer {
    separators: HashSet<char>
}

impl Tokenizer {
    pub fn new(separators: HashSet<char>) -> Self {
        Tokenizer { separators }
    }

    pub fn default() -> Self {
        let separators: HashSet<char> = [' ', '\n', '\r', '\t'].iter().cloned().collect();

        Tokenizer::new(separators)
    }

    pub fn tokenize<'a>(&self, input: &'a str) -> Vec<Token<'a>> {
        let mut output = Vec::new();
        let mut remaining = input;

        loop {
            let TokenResult { token, remaining } = read_token(&self.separators, remaining);

            if token.is_empty() {
                break;
            }

            output.push(token);
        }

        output
    }
}

pub struct Token<'a> {
    pub value: &'a str,
    pub suffix: &'a str
}

impl<'a> Token<'a> {
    fn is_empty(&self) -> bool {
        self.value.is_empty() && self.suffix.is_empty()
    }
}

fn read_token<'a, 'b>(separators: &'a HashSet<char>, input: &'b str) -> TokenResult<'b> {
    let mut iter = input.char_indices();

    let mut token_end = 0;

    while let Some((index, c)) = iter.next() {
        if separators.contains(&c) {
            token_end = index;
            break;
        }
    }

    let mut separator_end = 0;

    while let Some((index, c)) = iter.next() {
        if !separators.contains(&c) {
            separator_end = index;
            break;
        }
    }

    TokenResult {
        token: Token {
            value: &input[0 .. token_end],
            suffix: &input[token_end .. separator_end]
        },
        remaining: &input[separator_end..]
    }
}

struct TokenResult<'a> {
    token: Token<'a>,
    remaining: &'a str
}