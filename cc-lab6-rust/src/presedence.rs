use parse::{Token, Presedence, ParseError};


//  $   (   +   *   a   )   >
//  --------------------------
//  0   0   2   4   4   4
//  0   5   1   3   5   0


pub fn pr(a: &Token, b: &Token) -> Presedence {
    if f(a) < g(b) { Presedence::Less }
    else if f(a) == g(b) { Presedence::Equal }
    else { Presedence::Greater }
}

fn f(a: &Token) -> i32 {
    match a {
        &Token::Mul(..)             => 5,
        &Token::Add(..)             => 3,
        &Token::Cmp(..)             => 1,
        &Token::LeftBrace           => 0,
        &Token::RightBrace          => 5,
        &Token::Identifier(..)      => 5,
        &Token::End                 => 0
    }
}

fn g(a: &Token) -> i32 {
    match a {
        &Token::Mul(..)             => 4,
        &Token::Add(..)             => 2,
        &Token::Cmp(..)             => 2,
        &Token::LeftBrace           => 6,
        &Token::RightBrace          => 0,
        &Token::Identifier(..)      => 6,
        &Token::End                 => 0
    }
}
