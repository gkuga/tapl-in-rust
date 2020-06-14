use self::LexerError::*;
use super::parser::Token::{self, *};
use super::support::Info;
use std::sync::mpsc::Sender;

struct StateFunction(fn(&mut Lexer) -> Option<StateFunction>);

pub struct Lexer {
    input: String,
    start: usize,
    pos: usize,
    width: usize,
    token_sender: Sender<Result<Token, LexerError>>,
    file: String,
    current_line: usize,
    current_column: usize,
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    InvalidCharacter(Info, char),
}

const LEFT_COMMENT: &str = "/*";
const RIGHT_COMMENT: &str = "*/";

impl Lexer {
    pub fn lex(file: String, s: String, sender: Sender<Result<Token, LexerError>>) {
        let mut lexer = Lexer {
            input: s,
            start: 0,
            pos: 0,
            width: 0,
            token_sender: sender,
            file: file,
            current_line: 1,
            current_column: 0,
        };
        lexer.run();
    }

    fn run(&mut self) {
        let mut state = Some(StateFunction(Lexer::lex_arith));
        while let Some(StateFunction(next_state)) = state {
            state = next_state(self)
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.pos >= self.input.len() {
            self.width = 0;
            return None;
        }

        let c = self.input[self.pos..].chars().next().unwrap();
        self.width = c.len_utf8();
        self.pos += self.width;
        if c == '\n' {
            self.current_line += 1;
            self.current_column = self.pos;
        }
        Some(c)
    }

    fn backup(&mut self) {
        self.pos -= self.width;
    }

    fn ignore(&mut self) {
        self.start = self.pos;
    }

    fn emit(&mut self, token: Token) {
        self.token_sender
            .send(Ok(token))
            .expect("Unable to send token on channel");
        self.start = self.pos;
    }

    fn emit_error(&mut self, err: LexerError) {
        self.token_sender
            .send(Err(err))
            .expect("Unable to send token on channel");
        self.start = self.pos;
    }

    #[allow(dead_code)]
    fn accept(&mut self, valid: &str) -> bool {
        let c = self.next().unwrap();
        if valid.contains(c) {
            return true;
        }
        self.backup();
        false
    }

    #[allow(dead_code)]
    fn accept_run(&mut self, valid: &str) {
        let c = self.next().unwrap();
        while valid.contains(c) {}
        self.backup();
    }

    fn accept_run_ascii_digit(&mut self) {
        while let Some(c) = self.next() {
            if c.is_ascii_digit() {
                continue;
            }
            break;
        }
        self.backup();
    }

    fn accept_run_ascii_alphanumeric(&mut self) {
        while let Some(c) = self.next() {
            if c.is_ascii_alphanumeric() {
                continue;
            }
            break;
        }
        self.backup();
    }

    fn is_left_comment(&self) -> bool {
        &self.input[self.start..(self.start + LEFT_COMMENT.len())] == LEFT_COMMENT
    }

    fn get_keyword(&self, seek: &str) -> Option<Token> {
        match seek {
            "if" => Some(IF(self.create_info())),
            "then" => Some(THEN(self.create_info())),
            "else" => Some(ELSE(self.create_info())),
            "true" => Some(TRUE(self.create_info())),
            "false" => Some(FALSE(self.create_info())),
            "succ" => Some(SUCC(self.create_info())),
            "pred" => Some(PRED(self.create_info())),
            "iszero" => Some(ISZERO(self.create_info())),
            "Bool" => Some(BOOL(self.create_info())),
            "Nat" => Some(NAT(self.create_info())),
            _ => None,
        }
    }

    fn create_info(&self) -> Info {
        Info::FileInfo(self.file.clone(), self.current_line, self.current_column)
    }

    fn lex_arith(l: &mut Lexer) -> Option<StateFunction> {
        while let Some(c) = l.next() {
            match c {
                c if c.is_whitespace() => l.ignore(),
                c if c.is_ascii_digit() => return Some(StateFunction(Lexer::lex_number)),
                c if c.is_ascii_alphabetic() => return Some(StateFunction(Lexer::lex_identifier)),
                ';' => {
                    l.emit(SEMI(l.create_info()));
                }
                '(' => {
                    l.emit(LPAREN(l.create_info()));
                }
                ')' => {
                    l.emit(RPAREN(l.create_info()));
                }
                '/' if l.is_left_comment() => {
                    l.next();
                    l.ignore();
                    return Some(StateFunction(Lexer::lex_comment));
                }
                c => {
                    l.emit_error(InvalidCharacter(l.create_info(), c));
                    return None;
                }
            }
        }
        l.emit(Token::EOF(l.create_info()));
        None
    }

    fn lex_number(l: &mut Lexer) -> Option<StateFunction> {
        l.accept_run_ascii_digit();
        l.emit(Token::INTV(l.create_info()));
        Some(StateFunction(Lexer::lex_arith))
    }

    fn lex_identifier(l: &mut Lexer) -> Option<StateFunction> {
        l.accept_run_ascii_alphanumeric();
        let word = &l.input[l.start..l.pos];
        if let Some(token) = l.get_keyword(word) {
            l.emit(token);
        }
        Some(StateFunction(Lexer::lex_arith))
    }

    fn lex_comment(l: &mut Lexer) -> Option<StateFunction> {
        if let Some(c) = l.next() {
            match c {
                '*' if &l.input[l.start..(l.start + RIGHT_COMMENT.len())] == RIGHT_COMMENT => {
                    l.next();
                    l.ignore();
                    return Some(StateFunction(Lexer::lex_arith));
                }
                _ => {
                    l.ignore();
                    return Some(StateFunction(Lexer::lex_comment));
                }
            }
        }
        // TODO: error
        None
    }
}
