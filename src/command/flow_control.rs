//! Commands to do with flow control, such as Loop

use super::Command;
use image::RgbImage;
use program::Program;
use turtle::Turtle;

/// A loop will run a given program for a certain specified number of times
pub struct Loop {
    program: Program,
    times: u32,
}

impl Loop {
    /// Takes the program to run and the number of times to run it
    pub fn new(program: Program, times: u32) -> Loop {
        Loop { program, times }
    }
}

impl Command for Loop {
    fn run(&self, turtle: &mut Turtle, image: &mut RgbImage) {
        for _ in 0..self.times {
            self.program.run(turtle, image);
        }
    }

    // NOTE: Currently there is no support for nested loops, token stream needs to
    // represent this as a single block instead and deal with nesting
    fn parse<'a>(
        tokens: &mut impl ::std::iter::Iterator<Item = &'a str>,
    ) -> Result<Box<Self>, ::program::ParseError> {
        // Get the number of times to loop
        let times = tokens
            .next()
            .ok_or(::program::ParseError::NotEnoughArguments)?
            .parse()
            .map_err(|_| ::program::ParseError::InvalidArguments)?;

        let program = {
            // The first token in the program
            let first = tokens
                .next()
                .ok_or(::program::ParseError::NotEnoughArguments)?;
            // We got the start of the code block
            if first.starts_with('[') {
                let mut block = vec![first.split_at(1).1];

                loop {
                    if let Some(token) = tokens.next() {
                        if !token.contains(']') {
                            block.push(token);
                        } else {
                            block.push(token.split_at(token.len() - 1).0);

                            break;
                        }
                    } else {
                        return Err(::program::ParseError::MissingClosingParenthesis); // TODO
                    }
                }

                println!("Loop tokens: {:?}", block);

                Program::parse_from_tokens(&mut block.into_iter())?
            // This contains the whole program within the square brackets except for the start and end token
            // let block = tokens.take_while_ref(|token| !token.contains(']'));
            // // If there isn't a next then the take while never encountered a closing ']'
            // let last = tokens.next().ok_or(::program::ParseError::MissingClosingParenthesis)?;
            //
            // let mut block_vec: Vec<&str> = block.collect();
            //
            // let mut inner_tokens = vec!(first);
            // inner_tokens.append(&mut block_vec);
            // inner_tokens.push(last);
            //
            // Program::parse_from_tokens(&mut inner_tokens.into_iter())?

            // Probably a neater way of doing thing, but there doesn't seems to be a way based on how iterators are implemented
            // Program::parse_from_tokens(
            //     &mut [first].into_iter()
            //     .chain(block)
            //     // .chain([&last].iter())
            // )
            } else {
                return Err(::program::ParseError::InvalidArguments);
            }
        };

        Ok(Box::new(Self::new(program, times)))
    }

    fn to_code(&self) -> String {
        format!("repeat {} [{}]", self.times, self.program.to_code())
    }
}
