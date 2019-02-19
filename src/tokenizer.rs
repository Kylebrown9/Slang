use std::collections::{ HashSet };

/**
 * The singletons set indicates what characters should always be a token by themselves
 * The separators set indicates what characters indicate the boundary between tokens
 */
pub struct Tokenizer {
    singletons: HashSet<char>,
    separators: HashSet<char>
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Token<'a> {
    pub value: &'a str,
    pub suffix: &'a str
}

impl<'a> Token<'a> {
    #[cfg(test)]
    pub fn new(value: &'a str, suffix: &'a str) -> Self {
        Token { value, suffix }
    }
}

impl Tokenizer {
    pub fn new(singletons: HashSet<char>, separators: HashSet<char>) -> Self {
        Tokenizer { singletons, separators }
    }

    pub fn default() -> Self {
        let singletons: HashSet<char> = ['[', ']', '{', '}', '(', ')', ',', ':', '#'].iter().cloned().collect();
        let separators: HashSet<char> = [' ', '\n', '\r', '\t'].iter().cloned().collect();

        Tokenizer::new(singletons, separators)
    }

    pub fn tokenize<'a>(&self, input: &'a str) -> Vec<Token<'a>> {
        let mut output = Vec::new();
        let mut start = input;

        loop {
            let ParseResult { contents: value, remaining: post_val } = self.read_value(start);
            let ParseResult { contents: suffix, remaining: post_suf } = self.read_suffix(post_val);

            if value.is_empty() && suffix.is_empty() {
                break;
            } else {
                output.push(Token { value, suffix });
            }

            start = post_suf;
        }

        output
    }

    #[inline]
    fn read_value<'a>(&self, input: &'a str) -> ParseResult<'a> {
        let mut iter = input.char_indices();

        let mut short_cut = false;
        let mut value_end = 0;

        if let Some((_, c)) = iter.next() {
            if self.separators.contains(&c) {
                short_cut = true;
            }

            if self.singletons.contains(&c) {
                short_cut = true;
                value_end = 1;
            }
        }

        if !short_cut {
            while let Some((index, c)) = iter.next() {
                if self.separators.contains(&c)
                    || self.singletons.contains(&c) {

                    value_end = index;
                    break;
                }
            }
        }

        ParseResult {
            contents: &input[ .. value_end],
            remaining: &input[value_end .. ]
        }
    }

    #[inline]
    fn read_suffix<'a>(&self, input: &'a str) -> ParseResult<'a> {
        let mut iter = input.char_indices();

        while let Some((index, c)) = iter.next() {
            if !self.separators.contains(&c) {
                return ParseResult {
                    contents: &input[ .. index],
                    remaining: &input[index .. ]
                };
            }
        }

        ParseResult {
            contents: input,
            remaining: &input[0 .. 0]
        }
    }
}

struct ParseResult<'a> {
    contents: &'a str,
    remaining: &'a str
}

#[cfg(test)]
mod tests {
    use super::{ Tokenizer, Token };

    fn tokenizer_case(input: &str, expected: Vec<Token>) {
        let tokenizer = Tokenizer::default();

        let actual_tokens = tokenizer.tokenize(input);

        assert_eq!(actual_tokens, expected);
    }
    
    #[test]
    fn keeps_singletons_separate() {
        let input = "{[,]}";

        let expected_tokens = vec![
            Token::new("{", ""),
            Token::new("[", ""),
            Token::new(",", ""),
            Token::new("]", ""),
            Token::new("}", "")
        ];

        tokenizer_case(input, expected_tokens);
    }

    #[test]
    fn preserves_suffixes() {
        let input = "a b\tc\n";

        let expected_tokens = vec![
            Token::new("a", " "),
            Token::new("b", "\t"),
            Token::new("c", "\n")
        ];

        tokenizer_case(input, expected_tokens);
    }
    
    #[test]
    fn function_like() {
        let input = "fn a(b: C, d: E) -> F { b + d }";

        let expected_tokens = vec![
            Token::new("fn", " "),
            Token::new("a",  ""),
            Token::new("(",  ""),
            Token::new("b",  ""),
            Token::new(":",  " "),
            Token::new("C",  ""),
            Token::new(",",  " "),
            Token::new("d",  ""),
            Token::new(":",  " "),
            Token::new("E",  ""),
            Token::new(")",  " "),
            Token::new("->", " "),
            Token::new("F",  " "),
            Token::new("{",  " "),
            Token::new("b",  " "),
            Token::new("+",  " "),
            Token::new("d",  " "),
            Token::new("}",  "")
        ];

        tokenizer_case(input, expected_tokens);
    }
}