extern crate cc_lab5;

use std::env;

use cc_lab5::parse::{ ParserState, Symbol, Token, MulOp, AddOp, CmpOp };
use cc_lab5::parse;


fn main() {
    let seq = read_tokens();
    let parse_res = parse::parse(&seq);
    println!("{:?}", parse_res);
}

fn read_tokens() -> Vec<Token> {
    let mut res: Vec<Token> = Vec::new();
    for arg in env::args().skip(1).take(1) {
        let input_chain = arg.split(' ');
        for ch in input_chain {
            res.push(match ch {
                "+" => Token::Add(AddOp::Add),
                "-" => Token::Add(AddOp::Sub),
                "(" => Token::LeftBrace,
                ")" => Token::RightBrace,
                "i" => Token::Identifier,
                "*" => Token::Mul(MulOp::Mul),
                "/" => Token::Mul(MulOp::Div),
                ">" => Token::Cmp(CmpOp::G),
                "<" => Token::Cmp(CmpOp::L),
                "<>" => Token::Cmp(CmpOp::NE),
                ">=" => Token::Cmp(CmpOp::GE),
                "<=" => Token::Cmp(CmpOp::LE),
                t @ _ => panic!("invalid token: {}", t)
            });
        }
    };
    res.push(Token::End);
    return res;
}
