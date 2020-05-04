use super::lexer::Lexer;
use super::syntax::{Command, Term};
use std::sync::mpsc::*;
use std::thread;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    //Keyword tokens
    IF,
    THEN,
    ELSE,
    TRUE,
    FALSE,
    SUCC,
    PRED,
    ISZERO,

    // Identifier and constant value tokens
    INTV,

    // Symbolic tokens
    LPAREN,
    RPAREN,
    SEMI,
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidCharacter,
    InvalidExpression,
    EmptyExpression,
}

pub fn parse(input: String) -> Result<Vec<Command>, ParseError> {
    let (sender, receiver) = channel();
    thread::spawn(move || {
        Lexer::lex(input, sender);
    });
    let mut parser = Parser {
        receiver: receiver,
        token: [None; 3],
        peek_count: 0,
    };
    parser.parse()
}

struct Parser {
    receiver: Receiver<Token>,
    token: [Option<Token>; 3],
    peek_count: usize,
}

impl Parser {
    fn parse(&mut self) -> Result<Vec<Command>, ParseError> {
        let mut commands = Vec::<Command>::new();
        loop {
            match self.peek() {
                Some(Token::EOF) => break,
                _ => {
                    if let Ok(command) = self.get_command() {
                        commands.push(command);
                    }
                }
            }
        }
        Ok(commands)
    }

    fn next(&mut self) -> Option<Token> {
        if self.peek_count > 0 {
            self.peek_count -= 1;
        } else {
            let token = self.receiver.recv().unwrap();
            self.token[0] = Some(token);
        }
        self.token[self.peek_count]
    }

    fn peek(&mut self) -> Option<Token> {
        if self.peek_count > 0 {
            return self.token[self.peek_count - 1];
        }
        self.peek_count = 1;
        let token = self.receiver.recv().unwrap();
        self.token[0] = Some(token);
        self.token[0]
    }

    fn get_command(&mut self) -> Result<Command, ParseError> {
        match self.get_term() {
            Ok(term) => {
                if let Some(Token::SEMI) = self.next() {}
                Ok(Command::Eval(term))
            }
            _ => Err(ParseError::InvalidExpression),
        }
    }

    fn get_term(&mut self) -> Result<Term, ParseError> {
        match self.next() {
            Some(Token::TRUE) => {
                return Ok(Term::True);
            }
            Some(Token::FALSE) => {
                return Ok(Term::False);
            }
            Some(Token::INTV) => {
                return Ok(Term::Zero);
            }
            Some(Token::IF) => {
                let term = self.get_if().unwrap();
                return Ok(term);
            }
            Some(Token::ISZERO) => {
                let term = self.get_iszero().unwrap();
                return Ok(term);
            }
            Some(Token::SUCC) => {
                let term = self.get_succ().unwrap();
                return Ok(term);
            }
            Some(Token::PRED) => {
                let term = self.get_pred().unwrap();
                return Ok(term);
            }
            Some(Token::LPAREN) => {
                let term = self.get_term().unwrap();
                if let Some(Token::RPAREN) = self.next() {}
                return Ok(term);
            }
            t => {
                eprintln!("{:?}", t);
                return Err(ParseError::InvalidExpression);
            }
        }
    }

    fn get_if(&mut self) -> Result<Term, ParseError> {
        let t1 = self.get_term().unwrap();
        if let Some(Token::THEN) = self.next() {}
        let t2 = self.get_term().unwrap();
        if let Some(Token::ELSE) = self.next() {}
        let t3 = self.get_term().unwrap();
        Ok(Term::If(Box::new(t1), Box::new(t2), Box::new(t3)))
    }

    fn get_iszero(&mut self) -> Result<Term, ParseError> {
        let t = self.get_term().unwrap();
        Ok(Term::IsZero(Box::new(t)))
    }

    fn get_succ(&mut self) -> Result<Term, ParseError> {
        let t = self.get_term().unwrap();
        Ok(Term::Succ(Box::new(t)))
    }

    fn get_pred(&mut self) -> Result<Term, ParseError> {
        let t = self.get_term().unwrap();
        Ok(Term::Pred(Box::new(t)))
    }
}
