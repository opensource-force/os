pub(crate) mod lex;
pub(crate) mod parser;

use std::io::{self, Write};
use lex::Lexer;
use parser::{Parser, ParsedTk};


fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let mut lexer = Lexer::new();
        lexer.src.input.extend(buf.chars());
        lexer.lex();

        let mut parser = Parser::new(lexer);
        let tokens = parser.parse();

        println!("{:?}", tokens);
    }
}