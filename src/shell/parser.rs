use crate::lex::{Lexer, TkType};

#[derive(Debug, Clone)]
pub enum ParsedTk {
    Cmd(String, Vec<String>),
    Pipe(usize, usize),
    InputRedirect(usize),
    OutputRedirect(usize)
}

#[derive(Debug)] 
pub struct Parser {
    pub lexer: Lexer
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Vec<ParsedTk> {
        let mut parsed_tks: Vec<ParsedTk> = Vec::new();

        while let Some(tk) = self.lexer.tokens.next() {
            match tk.tk_type {
                TkType::Pipe => {
                    let prev_tk = parsed_tks.len() - 1;
                    let next_tk = parsed_tks.len() + 1;

                    parsed_tks.push(ParsedTk::Pipe(prev_tk, next_tk));
                },
                TkType::Lexeme => {
                    let cmd = tk.lexeme;
                    let mut args = vec![];

                    while let Some(tk) = self.lexer.tokens.peek() {
                        match tk.tk_type {
                            TkType::Lexeme => {
                                args.push(tk.lexeme.clone());
                                self.lexer.tokens.next();
                            },
                            _ => break
                        }
                    }

                    parsed_tks.push(ParsedTk::Cmd(cmd, args));
                },
                _ => println!("Not parsed!")
            }
        }

        parsed_tks
    }
}