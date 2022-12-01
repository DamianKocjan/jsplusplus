mod lexer;
mod parser;
mod syntax;

fn main() {
    let result = parser::parse("F:/projects/js++/test.js");

    if result.is_ok() {
        println!("Success!");
    } else {
        println!("Error: {}", result.err().unwrap());
    }

    println!("Hello, world!");
}
