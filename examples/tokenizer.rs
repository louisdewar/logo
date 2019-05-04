use logo_lib::parse::tokenizer::tokenize;

fn main() {
    let tokens = tokenize("fd 100 rt 45 repeat 4 [rt 90 fd 50]").unwrap();

    println!("{}", tokens);
    println!("{:?}", tokens);
}
