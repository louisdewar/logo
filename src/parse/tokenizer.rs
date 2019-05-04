use std::fmt;
use std::iter::Iterator;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Word(&'a str),
    Program(ProgramTokens<'a>),
}

#[macro_export]
macro_rules! try_word_token {
    ($expr:expr) => {
        match $expr {
            Token::Word(word) => word,
            Token::Program(program) => {
                return Err(crate::parse::ParseError::UnexpectedProgramArgument(program))
            }
        }
    };
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use colored::*;
        match self {
            Token::Word(word) => write!(f, "{}", word.blue().underline()),
            Token::Program(program_tokens) => write!(
                f,
                "{}{}{}",
                "[".bold().green(),
                program_tokens,
                "]".bold().green()
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramTokens<'a>(pub Vec<Token<'a>>);

impl<'a> fmt::Display for ProgramTokens<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use itertools;
        write!(f, "{}", itertools::join(self.iter(), " "))
    }
}

use std::convert;
impl<'a> convert::Into<Vec<Token<'a>>> for ProgramTokens<'a> {
    fn into(self) -> Vec<Token<'a>> {
        self.0
    }
}

use std::ops::Deref;
impl<'a> Deref for ProgramTokens<'a> {
    type Target = Vec<Token<'a>>;

    fn deref(&self) -> &Vec<Token<'a>> {
        &self.0
    }
}

use std::convert::From;
impl<'a> From<Vec<Token<'a>>> for ProgramTokens<'a> {
    fn from(vec: Vec<Token<'a>>) -> Self {
        ProgramTokens(vec)
    }
}

fn tokenize_block<'a>(
    last_index: &mut usize,
    input: &'a str,
    chars: &mut impl Iterator<Item = (usize, char)>,
    expecting_close: bool,
) -> Result<ProgramTokens<'a>, SyntaxError> {
    let mut tokens = vec![];

    while let Some((i, c)) = chars.next() {
        match c {
            // Found end of word
            ' ' | '\n' => {
                // Avoids adding empty words
                if *last_index != i {
                    tokens.push(Token::Word(&input[*last_index..i]));
                }

                // Start the index *after* the space hence `i + 1`
                *last_index = i + 1;
            }

            // Found sub-block
            '[' => {
                *last_index = i + 1;
                tokens.push(Token::Program(tokenize_block(
                    last_index, input, chars, true,
                )?))
            }

            // Found end
            ']' => {
                // Avoids adding empty words
                if *last_index != i {
                    // Add the last word before the ']'
                    tokens.push(Token::Word(&input[*last_index..i]));
                }

                // Set last index. If this is nested then it allows the parent to know the index of the end of the block
                *last_index = i + 1;

                if expecting_close {
                    return Ok(ProgramTokens(tokens));
                } else {
                    return Err(SyntaxError::UnexpectedToken(']'));
                }
            }

            // Other characters
            _ => {}
        }
    }

    if expecting_close {
        Err(SyntaxError::MissingClosingParenthesis)
    } else {
        // Add the final word
        let last = &input[*last_index..];
        if !last.is_empty() {
            tokens.push(Token::Word(&input[*last_index..]));
        }

        Ok(ProgramTokens(tokens))
    }
}

pub fn tokenize(input: &str) -> Result<ProgramTokens, SyntaxError> {
    tokenize_block(&mut 0, input, &mut input.char_indices(), false)
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxError {
    UnexpectedToken(char),
    MissingClosingParenthesis,
}
