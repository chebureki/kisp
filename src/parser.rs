use std::borrow::Borrow;
use std::iter::Peekable;
use crate::ast;
use crate::lexer::{Cursor, Token, TokenIterator, TokenValue, Lexer};
use crate::ast::SExpression;
use crate::lexer::TokenStream;
use crate::parser::ParserError::NoMatchingParser;

#[derive(Debug)]
pub enum ParserError{
    UnexpectedToken(Cursor),
    NoMatchingParser(Cursor),
    UnclosedParenthesis,
    //UnclosedParenthesis(Cursor),
}

pub type ParserResult = Result<Option<ast::SExpression>, ParserError>;
type Parser = fn(&mut TokenStream) -> ParserResult;

fn parse_iter(stream: &mut TokenStream, acc: Vec<SExpression>) -> Result<Vec<SExpression>, ParserError>{
    let mut acc = acc;
    match parse_s_expression(stream) {
        Ok(Some(e)) => {acc.push(e); parse_iter(stream, acc)},
        Ok(None) => Ok(acc),
        Err(e) => Err(e),
    }
}
pub fn parse(stream: &mut TokenStream) -> Result<SExpression, ParserError> {
    let stack = parse_iter(stream, Vec::new())?;
    match stream.peek().unwrap() {
        Token{value: TokenValue::EOF, ..} => Ok(SExpression::Block(stack)),
        Token{value, cursor} => Err(ParserError::NoMatchingParser(cursor.clone())),
    }
}

fn parse_s_expression(stream: &mut TokenStream) -> ParserResult {
    let parsers = [parse_atomic, parse_list, parse_block];
    for parser in parsers{
        match parser(stream){
            Ok(Some(data)) => return Ok(Some(data)),
            Err(e) => return Err(e),
            Ok(None) => {/* attempt next parser */}
        }
    }
    Ok(None)
}

fn parse_atomic(stream: &mut TokenStream) -> ParserResult{
    let mut stream = stream;
    match stream.next_if(|token| matches!(token.value, TokenValue::IntToken(_)) ||matches!(token.value, TokenValue::Identifier(_))) {
        Some(Token {value: TokenValue::Identifier(ident), ..}) => {
            Ok(Some(SExpression::Symbol(ident)))
        },
        Some(Token{value: TokenValue::IntToken(i), ..}) => {
            Ok(Some(SExpression::Number(i)))
        }
        _ => { Ok(None) }
    }
}

fn parse_list_iter(stream: &mut TokenStream, acc: Vec<ast::SExpression>) -> Result<Vec<ast::SExpression>, ParserError> {
    let mut acc = acc;
    match parse_s_expression(stream){
        Ok(Some(exp)) => {
            acc.push(exp);
            parse_list_iter(stream,acc)
        },
        Ok(None) => Ok(acc),
        Err(e) => Err(e)
    }
}

fn parse_list(stream: &mut TokenStream) -> ParserResult{
    match parse_listy(stream, TokenValue::ParenthesisOpen, TokenValue::ParenthesisClose)? {
        None => Ok(None),
        Some(acc) => Ok(Some(ast::SExpression::List(acc))),
    }
}

fn parse_block(stream: &mut TokenStream) -> ParserResult{
    match parse_listy(stream, TokenValue::BracketOpen, TokenValue::BracketClose)? {
        None => Ok(None),
        Some(acc) => Ok(Some(ast::SExpression::Block(acc))),
    }
}

fn parse_listy(stream: &mut TokenStream, open: TokenValue, close: TokenValue) -> Result<Option<Vec<SExpression>>, ParserError>{
    if stream.peek().unwrap().value != open {
        return Ok(None);
    }
    stream.next(); //discard open
    let inner = parse_list_iter(stream, Vec::new())?;
    if stream.peek().unwrap().value != close {
        return Err(ParserError::UnclosedParenthesis); //TODO: not generic enough
    }
    stream.next(); //discard close
    Ok(Some(inner))
}