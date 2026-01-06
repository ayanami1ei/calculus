use std::f64::consts::PI;

use crate::{expr::Token, tokenlizer::Tokenlizer};

impl Tokenlizer {
    fn tokenlize_alpha(&mut self, text: &[char]) -> Token {
        let mut res = String::new();

        while self.i < text.len() && text[self.i].is_alphabetic() {
            res.push(text[self.i]);
            self.i += 1;
        }

        if res == "log" {
            return Token::Log();
        } else if res=="sin"  || res=="cos"|| res=="tan"|| res=="arcsin"|| res=="arccos"|| res=="arctan"|| res=="csc"|| res=="sec"|| res=="cot"{
            return Token::Trifuncs(res);
        } else if res=="pi"{
            return Token::Const(PI);
        }

        Token::Identifier(res)
    }

    fn tokenlize_number(&mut self, text: &[char]) -> Result<Token, anyhow::Error> {
        let mut res = 0.0;

        while self.i < text.len() && text[self.i].is_ascii_digit() {
            res += text[self.i].to_string().parse::<f64>()?;
            self.i += 1;
        }

        Ok(Token::Const(res))
    }

    fn tokenlize_operator(&self, c: char) -> Token {
        return Token::Operator(c);
    }

    fn is_operator(c: char) -> bool {
        return c == '+'
            || c == '-'
            || c == '/'
            || c == '*'
            || c == '^'
            || c == '('
            || c == ')'
            || c == '['
            || c == ']'
            || c == '{'
            || c == '}'
            || c == '=';
    }

    pub fn tokenlize(&mut self) -> Result<Vec<Token>, anyhow::Error> {
        let mut tokens = Vec::<Token>::new();
        let text: Vec<char> = self.orig_text.chars().collect();

        while self.i < text.len() {
            if text[self.i].is_alphabetic() {
                tokens.push(self.tokenlize_alpha(&text));
                continue;
            } else if text[self.i].is_ascii_digit() {
                tokens.push(self.tokenlize_number(&text)?);
                continue;
            } else if Self::is_operator(text[self.i]) {
                tokens.push(self.tokenlize_operator(text[self.i]));
            } else if text[self.i].is_whitespace() || text[self.i] == ',' {
                self.i += 1;
                continue;
            } else {
                return Err(anyhow::Error::msg("unknown token type"));
            }

            self.i += 1;
        }

        Ok(tokens)
    }

    pub fn new(text: &String) -> Tokenlizer {
        Tokenlizer {
            orig_text: text.clone(),
            i:0,
        }
    }
}
