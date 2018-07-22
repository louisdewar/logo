use program::Program;

#[test]
fn parse_basic() {
    // This test should contain every command, to make sure that it is parsed and then it is able to output the original code
    let code = "fd 100 rt 90 fd 300 set_colour 25 50 255 set_pos 50 50 repeat 50 [pu fd 20 pd rt 45 fd 50]";
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
