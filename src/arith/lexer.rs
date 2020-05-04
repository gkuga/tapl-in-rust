use crate::arith::parser::Token;
use std::sync::mpsc::Sender;

struct StateFunction(fn(&mut Lexer) -> Option<StateFunction>);

pub struct Lexer {
    input: String,
    start: usize,
    pos: usize,
    width: usize,
    token_sender: Sender<Token>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum LexerError {
    InvalidCharacter((usize, char)),
    InvalidExpression,
    EmptyExpression,
}

const LEFT_COMMENT: &str = "/*";
const RIGHT_COMMENT: &str = "*/";

impl Lexer {
    pub fn lex(s: String, sender: Sender<Token>) {
        let mut lexer = Lexer {
            input: s,
            start: 0,
            pos: 0,
            width: 0,
            token_sender: sender,
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
            .send(token)
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

    fn accept_run_numeric(&mut self) {
        while let Some(c) = self.next() {
            if c.is_numeric() {
                continue;
            }
            break;
        }
        self.backup();
    }

    fn accept_run_alphanumeric(&mut self) {
        while let Some(c) = self.next() {
            if c.is_alphanumeric() {
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
            "if" => Some(Token::IF),
            "then" => Some(Token::THEN),
            "else" => Some(Token::ELSE),
            "true" => Some(Token::TRUE),
            "false" => Some(Token::FALSE),
            "succ" => Some(Token::SUCC),
            "pred" => Some(Token::PRED),
            "iszero" => Some(Token::ISZERO),
            _ => None,
        }
    }

    fn lex_arith(l: &mut Lexer) -> Option<StateFunction> {
        while let Some(c) = l.next() {
            match c {
                c if c.is_whitespace() => l.ignore(),
                c if c.is_numeric() => return Some(StateFunction(Lexer::lex_number)),
                c if c.is_alphanumeric() => return Some(StateFunction(Lexer::lex_identifier)),
                ';' => {
                    l.emit(Token::SEMI);
                }
                '(' => {
                    l.emit(Token::LPAREN);
                }
                ')' => {
                    l.emit(Token::RPAREN);
                }
                '/' if l.is_left_comment() => {
                    l.next();
                    l.ignore();
                    return Some(StateFunction(Lexer::lex_comment));
                }
                _ => {
                    // TODO: emit invalid character error
                    l.ignore()
                }
            }
        }
        l.emit(Token::EOF);
        None
    }

    fn lex_number(l: &mut Lexer) -> Option<StateFunction> {
        l.accept_run_numeric();
        l.emit(Token::INTV);
        Some(StateFunction(Lexer::lex_arith))
    }

    fn lex_identifier(l: &mut Lexer) -> Option<StateFunction> {
        l.accept_run_alphanumeric();
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
