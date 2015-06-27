extern crate cc_lab5;

use cc_lab5::parse::{ ParserState, Symbol, Token, MulOp, AddOp };
use cc_lab5::parse;

fn main() {
    let seq = [
        Token::LeftBrace,
        Token::Identifier,
        Token::Add(AddOp::Sub),
        Token::Identifier,
        Token::RightBrace,
        Token::Mul(MulOp::Div),
        Token::Identifier,
        Token::End
    ];

    let parse_res = parse::parse(&seq);
    println!("{:?}", parse_res);
}
