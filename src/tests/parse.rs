use program::Program;

#[test]
fn parse_basic() {
    let code = "fd 100 rt 90 fd 300";
    let result = Program::parse(code);

    match result {
        Ok(program) => {
            let reverse_code = program.to_code();
            assert!(
                reverse_code == code,
                "Decompiled code was not the same as the input. Got: `{}`, expected: `{}`",
                reverse_code,
                code
            );
        }
        Err(e) => panic!("Program could not be compiled. Err: {:?}", e),
    };
}
