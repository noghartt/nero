use std::fs;
use std::path::Path;

mod lexer;
mod parser;

use lexer::lexer::Lexer;
use parser::parser::Parser;

fn main() {
    let file = "/Users/noghartt/dev/nero/example/sum.no";
    let path = Path::new(file);

    match fs::read_to_string(path) {
        Ok(s) => {
            let mut x = Lexer::from_string(s);
            match x.scan() {
                Ok(x) => {
                    let mut parser = Parser::new(x);
                    let ast = parser.parse();

                    println!("{:#?}", ast);
                }
                Err(e) => println!("{:?}", e),
            }
        }
        Err(_) => println!("Something wrong while reading this path: {}", file),
    }
}
