use std::path::Path;
use std::fs;

mod lexer;
use lexer::lexer::Lexer;

fn main() {
    let file = "/Users/noghartt/dev/nero/example/sum.no";
    let path = Path::new(file);

    match fs::read_to_string(path) {
        Ok(s) => {
            let mut x = Lexer::from_string(s);
            match x.scan() {
                Ok(x) => println!("{:?}", x),
                Err(e) => println!("{:?}", e),
            }
        },
        Err(_) => println!("Something wrong while reading this path: {}", file),
    }
}
