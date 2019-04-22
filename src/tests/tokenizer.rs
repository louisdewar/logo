use parse::tokenizer::{tokenize, SyntaxError, Token};

#[test]
fn tokenizer_basic() {
    let code = "fd 100 rt 90 fd 300";
    let result = tokenize(code);

    match result {
        Ok(tokens) => {
            let manual_tokens = vec![
                Token::Word("fd"),
                Token::Word("100"),
                Token::Word("rt"),
                Token::Word("90"),
                Token::Word("fd"),
                Token::Word("300"),
            ];

            assert!(
                *tokens == manual_tokens,
                "Incorrect tokenization. Got: `{:?}`, expected: `{:?}`",
                tokens,
                manual_tokens
            );
        }
        Err(e) => panic!("Program `{}` could not be compiled. Err: {:?}", code, e),
    };
}

#[test]
fn tokenizer_loop() {
    let code = "fd 100 repeat 3 [rt 90 fd 300]";
    let result = tokenize(code);

    match result {
        Ok(tokens) => {
            let manual_tokens = vec![
                Token::Word("fd"),
                Token::Word("100"),
                Token::Word("repeat"),
                Token::Word("3"),
                Token::Program(
                    vec![
                        Token::Word("rt"),
                        Token::Word("90"),
                        Token::Word("fd"),
                        Token::Word("300"),
                    ]
                    .into(),
                ),
            ];

            assert!(
                *tokens == manual_tokens,
                "Incorrect tokenization. Got: `{:?}`, expected: `{:?}`",
                tokens,
                manual_tokens
            );
        }
        Err(e) => panic!("Program `{}` could not be compiled. Err: {:?}", code, e),
    };
}

#[test]
// A more advance use of a non-nested loop (to catch edge-cases)
fn tokenizer_loop_2() {
    let code = "fd 100 repeat 3 [rt 90 fd 300] fd 100 repeat 100 [rt 10 fd 50]";
    let result = tokenize(code);

    match result {
        Ok(tokens) => {
            let manual_tokens = vec![
                Token::Word("fd"),
                Token::Word("100"),
                Token::Word("repeat"),
                Token::Word("3"),
                Token::Program(
                    vec![
                        Token::Word("rt"),
                        Token::Word("90"),
                        Token::Word("fd"),
                        Token::Word("300"),
                    ]
                    .into(),
                ),
                Token::Word("fd"),
                Token::Word("100"),
                Token::Word("repeat"),
                Token::Word("100"),
                Token::Program(
                    vec![
                        Token::Word("rt"),
                        Token::Word("10"),
                        Token::Word("fd"),
                        Token::Word("50"),
                    ]
                    .into(),
                ),
            ];

            assert!(
                *tokens == manual_tokens,
                "Incorrect tokenization. Got: `{:?}`, expected: `{:?}`",
                tokens,
                manual_tokens
            );
        }
        Err(e) => panic!("Program `{}` could not be compiled. Err: {:?}", code, e),
    };
}

#[test]
fn tokenizer_nested_loop() {
    let code = "fd 100 repeat 3 [rt 90 repeat 5 [fd 300]]";
    let result = tokenize(code);

    match result {
        Ok(tokens) => {
            let manual_tokens = vec![
                Token::Word("fd"),
                Token::Word("100"),
                Token::Word("repeat"),
                Token::Word("3"),
                Token::Program(
                    vec![
                        Token::Word("rt"),
                        Token::Word("90"),
                        Token::Word("repeat"),
                        Token::Word("5"),
                        Token::Program(vec![Token::Word("fd"), Token::Word("300")].into()),
                    ]
                    .into(),
                ),
            ];

            assert!(
                *tokens == manual_tokens,
                "Incorrect tokenization. Got: `{:?}`, expected: `{:?}`",
                tokens,
                manual_tokens
            );
        }
        Err(e) => panic!("Program `{}` could not be compiled. Err: {:?}", code, e),
    };
}

#[test]
fn tokenizer_error_unexpected_end() {
    // Variation 1
    let code = "fd 100 repeat 3 [rt 90 repeat 5 [fd 300]]]";
    let err = tokenize(code).expect_err("Variation 1: Code with invalid syntax was tokenized");

    match err {
        SyntaxError::UnexpectedToken(token) => assert!(
            token == ']',
            "Variation 1: Wrong unexpected token, should have been ']' got '{}'",
            token
        ),
        _ => panic!("Variation 1: Got wrong error: {:?}", err),
    }

    // Variation 2
    let code = "fd 100 repeat 3 [rt 90 repeat 5 [fd 300]] fd 10]";
    let err = tokenize(code).expect_err("Variation 2: Code with invalid syntax was tokenized");

    match err {
        SyntaxError::UnexpectedToken(token) => assert!(
            token == ']',
            "Variation 2: Wrong unexpected token, should have been ']' got '{}'",
            token
        ),
        _ => panic!("Variation 2: Got wrong error: {:?}", err),
    }
}

#[test]
fn tokenizer_error_missing_end() {
    let code = "fd 100 repeat 3 [rt 90 repeat 5 [fd 300]";
    let err = tokenize(code).expect_err("Variation 1: Code with invalid syntax was tokenized");

    match err {
        SyntaxError::MissingClosingParenthesis => {}
        _ => panic!("Got wrong error: {:?}", err),
    }
}
