#![feature(generic_associated_types)]

use std::iter::Peekable;

mod langchars {
    pub const PARENTHESIS_OPEN: char = '(';
    pub const PARENTHESIS_CLOSE: char = ')';
    //const WHITESPACES: [char; 2] = [' ', '\t'];
    pub const SPACE: char = ' ';
    pub const TAB: char = '\t';
    pub const NEW_LINE: char = '\n';

    //disallowed in identifiers
    pub const NON_IDENTIFIER_CHARS: [char; 5] = [PARENTHESIS_OPEN, PARENTHESIS_CLOSE, SPACE, TAB, NEW_LINE];
}

#[derive(Clone, Debug)]
pub struct Cursor{
    line: usize,
    column: usize,
    abs_position: usize,
    reach: Option<usize>,
}

impl Cursor{
    fn new() -> Cursor {
        Cursor{abs_position: 0, line:1, column: 1, reach: None}
    }
    fn next_column(&self) -> Cursor {
        self.next_columns(1)
    }
    fn next_columns(&self, n: usize) -> Cursor {
        let mut clone = self.clone();
        clone.column+=n;
        clone.abs_position +=n;
        clone
    }

    fn next_line(&self) -> Cursor{
        let mut clone = self.clone();
        clone.line+=1;
        clone.column=1;
        clone.abs_position +=1;
        clone
    }
}

pub struct Lexer<'a>{
    txt_buffer: &'a str,
}
/*
pub enum Keyword{

}
 */

#[derive(Debug)]
pub enum TokenValue{
    Identifier(String),
    IntToken(i32),
    //StringLiteral(String),
    ParenthesisOpen,
    ParenthesisClose,
    EOF,
}

#[derive(Debug)]
pub struct Token{
    pub cursor: Cursor,
    pub value: TokenValue,
}

pub struct TokenIterator<'a> {
    lexer: Lexer<'a>,
    cursor: Cursor
}


impl<'t> Lexer<'t>{
    pub fn from_file_path(path: &str) -> Lexer {
        todo!()
    }

    pub fn from_text(data: &str) -> Lexer {
        Lexer{txt_buffer: data}
    }

    fn char_at(&self, i: usize) -> Option<char> {
        self.txt_buffer.chars().nth(i)
    }

    fn char_at_cursor(&self, cursor: &Cursor) -> Option<char>{
        self.char_at(cursor.abs_position)
    }

    fn is_identifier_char(c: char) -> bool {
        //kind an anti-pattern but yolo
        !langchars::NON_IDENTIFIER_CHARS.contains(&c)
    }

    fn possible_identifier_upgrade(v: &String) -> Option<TokenValue> {
        v.parse::<i32>().map(|v| Some(TokenValue::IntToken(v))).unwrap_or(None)
    }

    fn read_identifier(&self, cursor: &Cursor) -> (TokenValue, Cursor) {
        let ident = self.txt_buffer.chars()
            .into_iter()
            .skip(cursor.abs_position)
            .take_while(|c| Lexer::is_identifier_char(*c))
            .collect::<String>()
            ;
        let len = ident.len(); //damn you borrow checker
        (TokenValue::Identifier(ident),cursor.next_columns(len))
    }


    pub fn next_token(&self, cursor: &Cursor) -> (Token, Cursor){
        if let Some(char) = self.char_at_cursor(&cursor){
            match char{
                langchars::SPACE | langchars::TAB => { self.next_token(&cursor.next_column()) }
                langchars::NEW_LINE => { self.next_token(&cursor.next_line()) }
                langchars::PARENTHESIS_OPEN => { (Token{cursor: cursor.next_column(), value: TokenValue::ParenthesisOpen}, cursor.next_column()) }
                langchars::PARENTHESIS_CLOSE => { (Token{cursor: cursor.next_column(), value: TokenValue::ParenthesisClose}, cursor.next_column()) }
                _ => {
                    let (ident_token, after_cursor) = self.read_identifier(cursor);
                    let TokenValue::Identifier(i) = ident_token else {panic!("didn't receive identifier")};
                    let value = Lexer::possible_identifier_upgrade(&i).unwrap_or(TokenValue::Identifier(i));
                    (
                        Token{
                        cursor: Cursor{reach: Some(after_cursor.abs_position-cursor.abs_position), ..*cursor},
                        value
                    },
                        after_cursor
                    )
                }
            }
        }
        else{
            (Token{cursor: cursor.clone(), value: TokenValue::EOF}, cursor.clone())
        }
    }
}

impl<'t> Iterator for TokenIterator<'t>{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let (token, next_cursor) = self.lexer.next_token(&self.cursor);
        self.cursor = next_cursor;
        Some(token)
    }
}

impl <'t>IntoIterator for Lexer<'t>{
    type Item = Token;
    type IntoIter = Peekable<TokenIterator<'t>>;

    fn into_iter(self) -> Self::IntoIter {
        TokenIterator{lexer: self, cursor: Cursor::new()}.peekable()
        //Peekable{TokenIterator{lexer: self, cursor: Cursor::new()}}
    }
}


pub type TokenStream<'a> = Peekable<TokenIterator<'a>>;