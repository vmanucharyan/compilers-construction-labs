extern crate cc_lab5;

use std::env;
use std::str::FromStr;
use std::iter::Filter;
use std::collections::VecDeque;

use cc_lab5::parse::{ ParserState, Symbol, Token, MulOp, AddOp, CmpOp };
use cc_lab5::parse;


fn main() {
    let seq = read_tokens();
    let parse_res = parse::parse(&seq);
    println!("{:?}", parse_res);

    if let ParserState::Accept(ref parse) = parse_res {
        println!("{}", prn(parse, &seq));
    }
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
                "*" => Token::Mul(MulOp::Mul),
                "/" => Token::Mul(MulOp::Div),
                ">" => Token::Cmp(CmpOp::G),
                "<" => Token::Cmp(CmpOp::L),
                "<>" => Token::Cmp(CmpOp::NE),
                ">=" => Token::Cmp(CmpOp::GE),
                "<=" => Token::Cmp(CmpOp::LE),
                t @ _ if i32::from_str(t).is_ok() => Token::Identifier(t.to_string()),
                t @ _ => panic!("invalid token: {}", t)
            });
        }
    };
    res.push(Token::End);
    return res;
}

fn prn(parse: &Vec<i32>, seq: &[Token]) -> String {
    let mut id_seq: VecDeque<String> = VecDeque::new();
    let mut mul_seq: VecDeque<&str> = VecDeque::new();
    let mut add_seq: VecDeque<&str> = VecDeque::new();
    let mut cmp_seq: VecDeque<&str> = VecDeque::new();

    for s in seq {
        match *s {
            Token::Identifier(ref id) => id_seq.push_back(id.clone()),
            Token::Add(AddOp::Add) => add_seq.push_back("+"),
            Token::Add(AddOp::Sub) => add_seq.push_back("-"),
            Token::Mul(MulOp::Mul) => mul_seq.push_back("*"),
            Token::Mul(MulOp::Div) => mul_seq.push_back("/"),
            Token::Cmp(CmpOp::G)   => cmp_seq.push_back(">"),
            Token::Cmp(CmpOp::L)   => cmp_seq.push_back("<"),
            Token::Cmp(CmpOp::NE)  => cmp_seq.push_back("<>"),
            Token::Cmp(CmpOp::GE)  => cmp_seq.push_back(">="),
            Token::Cmp(CmpOp::LE)  => cmp_seq.push_back("<="),
            _ => {}
        }
    }

    let mut res = String::new();
    for p in parse {
        match *p {
            7 => res.push_str(&id_seq.pop_front().unwrap()),
            2 => res.push_str(&add_seq.pop_front().unwrap()),
            4 => res.push_str(&mul_seq.pop_front().unwrap()),
            1 => res.push_str(&cmp_seq.pop_front().unwrap()),
            _ => {}
        }
        match *p {
            7 | 2 | 4 | 1 => res.push_str(" "),
            _ => {}
        }
    }

    return res;
}
