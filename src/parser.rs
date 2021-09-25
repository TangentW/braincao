use std::slice::Iter;
use crate::buffer::Offset;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidInput(usize),
    LoopNotCompatible
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(pos) =>
                write!(f, "invalid input at {}", pos),
            Self::LoopNotCompatible =>
                write!(f, "loop not compatible")
        }
    }
}

#[derive(Debug)]
pub(crate) enum Expression {
    ValOffset(Offset<i32>),
    PtrOffset(Offset<usize>),
    Input,
    Output,
    Loop(Vec<Expression>)
}

pub(crate) fn parse(input: impl Iterator<Item=char>) -> Result<Vec<Expression>, Error> {
    let tokens = tokenize(input);
    let mut tokens_iter = tokens.iter();
    parse_tokens(&mut tokens_iter, &mut 0)
}

fn parse_tokens(tokens: &mut Iter<Token>, loop_count: &mut i8) -> Result<Vec<Expression>, Error> {
    let mut expressions = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
            Token::Unknown(pos) => return Err(Error::InvalidInput(*pos)),

            Token::Forward => ptr_offset(&mut expressions, Offset::Inc(1)),
            Token::Backward => ptr_offset(&mut expressions, Offset::Dec(1)),
            Token::Inc => val_offset(&mut expressions, Offset::Inc(1)),
            Token::Dec => val_offset(&mut expressions, Offset::Dec(1)),

            Token::Input => expressions.push(Expression::Input),
            Token::Output => expressions.push(Expression::Output),

            Token::LoopStart => {
                *loop_count += 1;
                expressions.push(Expression::Loop(parse_tokens(tokens, loop_count)?));
            },
            Token::LoopEnd => {
                *loop_count -= 1;
                return if *loop_count < 0 {
                    Err(Error::LoopNotCompatible)
                } else {
                    Ok(expressions)
                }
            }
        }
    }
    return if *loop_count < 0 {
        Err(Error::LoopNotCompatible)
    } else {
        Ok(expressions)
    }
}

fn val_offset(expressions: &mut Vec<Expression>, offset: Offset<i32>) {
    match (expressions.last_mut(), offset) {
        (Some(Expression::ValOffset(Offset::Inc(val))), Offset::Inc(offset)) => *val += offset,
        (Some(Expression::ValOffset(Offset::Dec(val))), Offset::Dec(offset)) => *val += offset,
        _ => expressions.push(Expression::ValOffset(offset)),
    }
}

fn ptr_offset(expressions: &mut Vec<Expression>, offset: Offset<usize>) {
    match (expressions.last_mut(), offset) {
        (Some(Expression::PtrOffset(Offset::Inc(val))), Offset::Inc(offset)) => *val += offset,
        (Some(Expression::PtrOffset(Offset::Dec(val))), Offset::Dec(offset)) => *val += offset,
        _ => expressions.push(Expression::PtrOffset(offset)),
    }
}

#[derive(Debug)]
enum Token {
    Forward,
    Backward,
    Inc,
    Dec,
    Input,
    Output,
    LoopStart,
    LoopEnd,
    Unknown(usize)
}

fn tokenize(input: impl Iterator<Item=char>) -> Vec<Token> {
    input.filter(|c| !c.is_ascii_whitespace())
        .enumerate()
        .map(|(pos, c)| match c {
            '>' => Token::Forward,
            '<' => Token::Backward,
            '+' => Token::Inc,
            '-' => Token::Dec,
            ',' => Token::Input,
            '.' => Token::Output,
            '[' => Token::LoopStart,
            ']' => Token::LoopEnd,
            _ => Token::Unknown(pos),
        })
        .collect()
}
