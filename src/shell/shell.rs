use std::error::Error;
use std::io::{self, Write};
use std::{fs, env};

use lex::{TkType, Lexer};

const AMBER: &str = "38;2;255;176;0m";
const NEON: &str = "38;2;102;255;102m";

fn main() -> Result<(), Box<dyn Error>> {
    let _home: &str = &env::var("HOME")?;
    let user: &str = &env::var("USER")?;
    let host = fs::read_to_string("/etc/hostname")?;

    //print!("\x1B[2J\x1B[3J\x1B[H");

    loop {
        let cwd = env::current_dir()?;

        print!("\x1B[{}{}\x1B[m\n{}\x1B[2m@{}\x1B[m \x1B[{}>\x1B[m: ",
            AMBER, cwd.to_string_lossy(), user, host.trim(), NEON);
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let mut lexer = Lexer::new();
        lexer.src.input.extend(buf.chars());
        lexer.lex();

        while let Some(token) = lexer.tokens.next() {
            match token.tk_type {
                TkType::Pipe => println!("Found Pipe"),
                TkType::Lexeme => {
                    match token.lexeme.as_str() {
                        "exit" => {
                            println!("Shell closed!");
                            return Ok(())
                        },
                        _ => println!("Input not parsed")
                    }
                }
            _ => {} // implement input/output redirection
            }
        }
    }
}