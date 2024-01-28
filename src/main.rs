use std::io::{self, Write};

#[derive(Clone, Copy)]
// open file as current -> tokenize
enum TKind {
    Operator,
    Number,
    EOF,
}
#[derive(Clone)]
struct Token {
    val: String,
    kind: TKind,
}

struct Source {
    code: String,
    line: u32,
    colmun: u32,
    current_tok: Option<Token>,
}

// impl Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let _ = write!(f, "{}:<{}>", self.val, self.kind);
//         return Ok(());
//     }
// }

// impl Display for TKind {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let _ = write!(f, "{}", self);
//         return Ok(());
//     }
// }

impl Source {
    pub fn new(code: String) -> Source {
        Source {
            code,
            line: 1,
            colmun: 1,
            current_tok: None,
        }
    }
}

fn is_num(c: char) -> bool {
    return "0123456789".contains(c);
}

trait Tokenizer {
    fn eat(&mut self) -> char;
    fn at(&self) -> char;
    fn set(&mut self, val: String, kind: TKind);
    fn current(&mut self) -> Token;
    fn tokenize(&mut self) -> Result<u32, String>;
}

impl Tokenizer for Source {
    fn eat(&mut self) -> char {
        let p = self.at();
        self.code.remove(0);
        return p;
    }

    fn at(&self) -> char {
        return self.code.as_bytes()[0] as char;
    }

    fn set(&mut self, val: String, kind: TKind) {
        self.current_tok = Some(Token { val, kind });
    }

    fn current(&mut self) -> Token {
        return self.current_tok.clone().expect("None");
    }

    fn tokenize(&mut self) -> Result<u32, String> {
        while self.code.len() > 0 && (self.at() == ' ' || self.at() == '\t' || self.at() == '\n') {
            while self.code.len() > 0 && (self.at() == ' ' || self.at() == '\t') {
                self.colmun += 1;
                self.eat();
                continue;
            }

            while self.code.len() > 0 && self.at() == '\n' {
                self.eat();
                self.colmun = 1;
                self.line += 1;
                continue;
            }
        }

        if self.code.len() <= 0 {
            self.set("<END>".to_string(), TKind::EOF);
            return Ok(0);
        }

        match self.at() {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut res = String::from("");
                while is_num(self.at()) {
                    res.push(self.eat())
                }
                self.set(res.to_string(), TKind::Number);
                return Ok(1);
            }
            '+' | '-' | '*' | '/' | '^' => {
                let op = self.eat();
                self.set(op.to_string(), TKind::Operator);
                return Ok(2);
            }
            _ => {
                if false {
                    return Err("how did we get here?".to_string());
                } else {
                    let c = self.eat();
                    return Err(format!("AT0001::UNKNOWN_CHAR_{}", c));
                }
            }
        }
    }
}

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::with_capacity(4096);
        let stdin = io::stdin();

        let _ = stdin.read_line(&mut buffer);
        let mut src = Source::new(buffer);

        println!("entered {}", src.code);
        let mut tokens: Vec<Token> = Vec::new();
        while src.tokenize() != Ok(0) {
            let t = src.current();
            println!("is {}:{}", t.val, t.kind as u32);
            tokens.push(t);
        }
    }
}
