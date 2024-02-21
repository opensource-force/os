pub enum TkType {
    Pipe,
    InputRedirect,
    OutputRedirect,
    Lexeme,
    EOF
}

pub struct Tk {
    pub tk_type: TkType,
    pub lexeme: String
}

impl Tk {
    pub fn new(tk_type: TkType, lexeme: Option<String>) -> Self {
        Self {
            tk_type,
            lexeme: lexeme.unwrap_or_default()
        }
    }
}

pub struct TkBuf {
    pub tks: Vec<Tk>,
    pub pos: usize
}

impl TkBuf {
    pub fn new() -> Self {
        Self {
            tks: vec![],
            pos: 0
        }
    }

    pub fn push(&mut self, tk: Tk) {
        self.tks.push(tk);
    }

    pub fn next(&mut self) -> Option<&Tk> {
        if self.pos < self.tks.len() {
            let c = &self.tks[self.pos];
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }
}

pub struct SrcBuf {
    pub input: Vec<char>,
    pub pos: usize,
}

impl SrcBuf {
    pub fn new() -> Self {
        Self {
            input: vec![],
            pos: 0
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if self.pos < self.input.len() {
            let c = self.input[self.pos];
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }
}

pub struct Lexer {
    pub src: SrcBuf,
    pub tokens: TkBuf
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            src: SrcBuf::new(),
            tokens: TkBuf::new()
        }
    }

    pub fn lex(&mut self) {
        while let Some(c) = self.src.next() {
            match c {
                ' ' | '\n' => continue,
                '|' => self.tokens.push(Tk::new(TkType::Pipe, None)),
                '<' => self.tokens.push(Tk::new(TkType::InputRedirect, None)),
                '>' => self.tokens.push(Tk::new(TkType::OutputRedirect, None)),
                'a'..='z' | 'A'..='Z' => {
                    let mut lexeme = String::new();
                    lexeme.push(c);

                    while let Some(l) = self.src.next() {
                        match l {
                            'a'..='z' | 'A'..='Z' | '0'..='9' => lexeme.push(l),
                            _ => break
                        }
                    }
                    self.tokens.push(Tk::new(TkType::Lexeme, Some(lexeme)));
                },
                _ => panic!("Bad character")
            }
        }

        self.tokens.push(Tk::new(TkType::EOF, None));
    }
}