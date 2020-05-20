use super::lexer::Lexer;
use super::lexer::LexerError;
use super::support::Info::{self, *};
use super::support::WithInfo;
use super::syntax::{get_info, name2index, Binding, Command, Term};
use std::sync::mpsc::*;
use std::thread;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    //Keyword tokens
    LAMBDA(Info),

    // Identifier and constant value tokens
    LCID(WithInfo),

    // Symbolic tokens
    SLASH(Info),
    DOT(Info),
    USCORE(Info),
    LPAREN(Info),
    RPAREN(Info),
    SEMI(Info),
    EOF(Info),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidToken(Info),
    MissingSemicolon(Info),
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
        context: Vec::new(),
    };
    parser.parse()
}

struct Parser {
    receiver: Receiver<Result<Token, LexerError>>,
    token: [Option<Token>; 3],
    peek_count: usize,
    context: Vec<(String, Binding)>,
}

impl Parser {
    fn parse(&mut self) -> Result<Vec<Command>, ParseError> {
        let mut commands = Vec::<Command>::new();
        loop {
            match self.peek() {
                Token::EOF(_) => break,
                _ => match self.get_command() {
                    Ok(command) => commands.push(command),
                    Err(e) => return Err(e),
                },
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

    fn backup(&mut self) {
        self.peek_count += 1;
    }

    fn backup2(&mut self, token: Token) {
        self.token[1] = Some(token);
        self.peek_count = 2;
    }

    fn get_command(&mut self) -> Result<Command, ParseError> {
        let t1 = self.next();
        let t2 = self.next();
        if let Token::LCID(WithInfo { i, v }) = t1.clone() {
            if let Token::SLASH(_) = t2 {
                if let Token::SEMI(_) = self.next() {
                    self.context.insert(0, (v.clone(), Binding::NameBind));
                    return Ok(Command::Bind(i, v, Binding::NameBind));
                } else {
                    return Err(ParseError::MissingSemicolon(Unknown));
                }
            }
        }
        self.backup2(t1);
        match self.get_term(self.context.clone()) {
            Ok(term) => {
                if let Token::SEMI(_) = self.next() {
                } else {
                    return Err(ParseError::MissingSemicolon(Unknown));
                }
                Ok(Command::Eval(get_info(&term), term))
            }
            Err(err) => Err(err),
        }
    }

    fn get_term(&mut self, mut context: Vec<(String, Binding)>) -> Result<Term, ParseError> {
        match self.next() {
            Token::LPAREN(_) | Token::LCID(_) => {
                self.backup();
                return self.get_app_term(context);
            }
            Token::LAMBDA(i) => {
                let s = match self.next() {
                    Token::LCID(WithInfo { i: _, v }) => v,
                    Token::USCORE(_) => String::from(" "),
                    _ => return Err(ParseError::InvalidToken(Unknown)),
                };
                if let Token::DOT(_) = self.next() {
                } else {
                    return Err(ParseError::InvalidToken(Unknown));
                }
                context.insert(0, (s.clone(), Binding::NameBind));
                return Ok(Term::Abs(i, s, Box::new(self.get_term(context)?)));
            }
            t => {
                eprintln!("{:?}", t);
                return Err(ParseError::InvalidToken(Unknown));
            }
        }
    }

    fn get_app_term(&mut self, context: Vec<(String, Binding)>) -> Result<Term, ParseError> {
        let aterm = self.get_aterm(context.clone())?;
        if let Token::LPAREN(_) | Token::LCID(_) = self.peek() {
        } else {
            return Ok(aterm);
        }

        let mut app_term = Term::App(
            get_info(&aterm),
            Box::new(aterm),
            Box::new(self.get_aterm(context.clone())?),
        );
        while let Token::LPAREN(_) | Token::LCID(_) = self.peek() {
            app_term = Term::App(
                get_info(&app_term),
                Box::new(app_term),
                Box::new(self.get_aterm(context.clone())?),
            );
        }
        return Ok(app_term);
    }

    fn get_aterm(&mut self, context: Vec<(String, Binding)>) -> Result<Term, ParseError> {
        match self.next() {
            Token::LPAREN(_) => {
                let term = self.get_term(context);
                if let Token::RPAREN(_) = self.next() {
                } else {
                    return Err(ParseError::InvalidToken(Unknown));
                }
                return term;
            }
            Token::LCID(WithInfo { i, v }) => {
                return Ok(Term::Var(
                    i.clone(),
                    name2index(i.clone(), context.as_slice(), v)?,
                    context.len(),
                ));
            }
            t => {
                eprintln!("{:?}", t);
                return Err(ParseError::InvalidToken(Unknown));
            }
        }
    }
}
