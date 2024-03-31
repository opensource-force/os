#[derive(Debug, Clone)] 
pub enum TkType {
    Pipe,
    InputRedirect,
    OutputRedirect,
    OutputRedirectAppend,
    StringLiteral,
    StringExpansion,
    Lexeme,
    EOF
}

#[derive(Debug, Clone)] 
pub struct Tk {
    pub tk_type: TkType,
    pub lexeme: String
}

impl Tk {
    fn new(tk_type: TkType, lexeme: Option<String>) -> Self {
        Self { tk_type, lexeme: lexeme.unwrap_or_default() }
    }

    pub fn get(&self) -> &str { self.lexeme.as_str() }
}

#[derive(Debug, Clone)] 
pub struct TkBuf {
    tks: Vec<Tk>,
    pos: usize,
}

impl TkBuf {
    fn new() -> Self { Self { tks: vec![], pos: 0 } }


    fn push(&mut self, tk: Tk) { self.tks.push(tk); }

    pub fn peek(&mut self) -> Option<&Tk> {
        if self.pos < self.tks.len() {
            return Some(&self.tks[self.pos])
        }

        None
    }

    pub fn next(&mut self) -> Option<Tk> {
        if self.pos < self.tks.len() {
            let tk = self.tks[self.pos].clone();
            self.pos += 1;

            return Some(tk)
        }

        None
    }

    pub fn get(&self) -> Vec<String> {
        self.tks.iter().map(|tk| tk.lexeme.clone()).collect()
    }

    pub fn last(&self) -> Option<String> {
        self.tks.last().map(|tk| tk.lexeme.clone())
    }

    pub fn repl_matches(&mut self, any: &str, repl: &str) {
        for tk in &mut self.tks {
            tk.lexeme = tk.lexeme.replace(any, repl);
        }
    }
}

#[derive(Debug, Clone)] 
pub struct SrcBuf {
    pub input: Vec<char>,
    pos: usize,
}

impl SrcBuf {
    fn new() -> Self { Self { input: vec![], pos: 0 } }

    fn peek(&self) -> Option<char> {
        if self.pos < self.input.len() {
            return Some(self.input[self.pos])
        }

        None
    }

    fn next(&mut self) -> Option<char> {
        if self.pos < self.input.len() {
            let c = self.input[self.pos];
            self.pos += 1;

            return Some(c)
        }
        
        None
    }
}

#[derive(Debug, Clone)] 
pub struct Lexer {
    pub src: SrcBuf,
    pub tokens: TkBuf
}

impl Lexer {
    pub fn new() -> Self {
        Self { src: SrcBuf::new(), tokens: TkBuf::new() }
    }

    pub fn lex(&mut self) {
        while let Some(c) = self.src.next() {
            match c {
                ' ' | '\n' => continue,
                '|' => self.tokens.push(Tk::new(TkType::Pipe, None)),
                '<' => self.tokens.push(Tk::new(TkType::InputRedirect, None)),
                '>' => {
                    if let Some(c) = self.src.peek() {
                        match c {
                            '>' => {
                                self.tokens.push(Tk::new(TkType::OutputRedirectAppend, None));
                                self.src.next();
                            }
                            _ => self.tokens.push(Tk::new(TkType::OutputRedirect, None))
                        }
                    }
                }
                '\'' => {
                    let mut string_literal = String::new();
                    string_literal.push(c);

                    while let Some(c) = self.src.next() {
                        string_literal.push(c);

                        if c == '\'' { break; }
                    }

                    self.tokens.push(Tk::new(TkType::StringLiteral, Some(string_literal)));
                }
                'a'..='z' | 'A'..='Z' | '/' | '.' | '~' | '\\' => {
                    let mut lexeme = String::new();
                    lexeme.push(c);

                    while let Some(c) = self.src.next() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '/' | '.' => lexeme.push(c),
                            _ => break
                        }
                    }
                    self.tokens.push(Tk::new(TkType::Lexeme, Some(lexeme)));
                },
                _ => panic!("Bad character")
            }
        }

        //self.tokens.push(Tk::new(TkType::EOF, None));
    }
}