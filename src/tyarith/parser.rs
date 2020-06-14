use super::lexer::Lexer;
use super::lexer::LexerError;
use super::support::Info::{self, *};
use super::syntax::{Command, Term};
use std::sync::mpsc::*;
use std::thread;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    //Keyword tokens
    IF(Info),
    THEN(Info),
    ELSE(Info),
    TRUE(Info),
    FALSE(Info),
    SUCC(Info),
    PRED(Info),
    ISZERO(Info),
    BOOL(Info),
    NAT(Info),

    // Identifier and constant value tokens
    INTV(Info),

    // Symbolic tokens
    LPAREN(Info),
    RPAREN(Info),
    SEMI(Info),
    EOF(Info),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidToken(Info),
    EmptyExpression(Info),
}

pub fn parse(file: String, input: String) -> Result<Vec<Command>, ParseError> {
    let (sender, receiver) = channel();
    thread::spawn(move || {
        Lexer::lex(file, input, sender);
    });
    let mut parser = Parser {
        receiver: receiver,
        token: [None, None, None],
        peek_count: 0,
    };
    parser.parse()
}

struct Parser {
    receiver: Receiver<Result<Token, LexerError>>,
    token: [Option<Token>; 3],
    peek_count: usize,
}

impl Parser {
    fn parse(&mut self) -> Result<Vec<Command>, ParseError> {
        let mut commands = Vec::<Command>::new();
        loop {
            match self.peek() {
                Token::EOF(_) => break,
                _ => {
                    if let Ok(command) = self.get_command() {
                        commands.push(command);
                    }
                }
            }
        }
        Ok(commands)
    }

    fn next(&mut self) -> Token {
        if self.peek_count > 0 {
            self.peek_count -= 1;
        } else {
            let token = self.receiver.recv().unwrap().unwrap();
            self.token[0] = Some(token);
        }
        self.token[self.peek_count].clone().unwrap()
    }

    fn peek(&mut self) -> Token {
        if self.peek_count > 0 {
            return self.token[self.peek_count - 1].clone().unwrap();
        }
        self.peek_count = 1;
        let token = self.receiver.recv().unwrap().unwrap();
        self.token[0] = Some(token);
        self.token[0].clone().unwrap()
    }

    fn get_command(&mut self) -> Result<Command, ParseError> {
        match self.get_term() {
            Ok(term) => {
                if let Token::SEMI(_) = self.next() {}
                Ok(Command::Eval(term))
            }
            Err(err) => Err(err),
        }
    }

    fn get_term(&mut self) -> Result<Term, ParseError> {
        match self.next() {
            Token::TRUE(i) => {
                return Ok(Term::True(i));
            }
            Token::FALSE(i) => {
                return Ok(Term::False(i));
            }
            Token::INTV(i) => {
                return Ok(Term::Zero(i));
            }
            Token::IF(i) => {
                let term = self.get_if(i).unwrap();
                return Ok(term);
            }
            Token::ISZERO(i) => {
                let term = self.get_iszero(i).unwrap();
                return Ok(term);
            }
            Token::SUCC(i) => {
                let term = self.get_succ(i).unwrap();
                return Ok(term);
            }
            Token::PRED(i) => {
                let term = self.get_pred(i).unwrap();
                return Ok(term);
            }
            Token::LPAREN(_) => {
                let term = self.get_term().unwrap();
                if let Token::RPAREN(_) = self.next() {}
                return Ok(term);
            }
            t => {
                eprintln!("{:?}", t);
                return Err(ParseError::InvalidToken(Unknown));
            }
        }
    }

    fn get_if(&mut self, i: Info) -> Result<Term, ParseError> {
        let t1 = self.get_term().unwrap();
        if let Token::THEN(_) = self.next() {}
        let t2 = self.get_term().unwrap();
        if let Token::ELSE(_) = self.next() {}
        let t3 = self.get_term().unwrap();
        Ok(Term::If(i, Box::new(t1), Box::new(t2), Box::new(t3)))
    }

    fn get_iszero(&mut self, i: Info) -> Result<Term, ParseError> {
        let t = self.get_term().unwrap();
        Ok(Term::IsZero(i, Box::new(t)))
    }

    fn get_succ(&mut self, i: Info) -> Result<Term, ParseError> {
        let t = self.get_term().unwrap();
        Ok(Term::Succ(i, Box::new(t)))
    }

    fn get_pred(&mut self, i: Info) -> Result<Term, ParseError> {
        let t = self.get_term().unwrap();
        Ok(Term::Pred(i, Box::new(t)))
    }
}
