use parse::{Token, Presedence, ParseError};

pub fn pr(a: &Token, b: &Token) -> Presedence {
    match (a, b) {
        (&Token::RightBrace, &Token::LeftBrace) => Presedence::Error(ParseError::OperandMissing),
        (&Token::RightBrace, &Token::Identifier(..)) => Presedence::Error(ParseError::OperatorMissing),
        (&Token::RightBrace, &Token::Mul(..)) => Presedence::Greater,
        (&Token::RightBrace, &Token::Add(..)) => Presedence::Greater,
        (&Token::RightBrace, &Token::Cmp(..)) => Presedence::Greater,
        (&Token::RightBrace, &Token::RightBrace) => Presedence::Greater,
        (&Token::RightBrace, &Token::End) => Presedence::Greater,

        (&Token::Identifier(..), &Token::LeftBrace) => Presedence::Error(ParseError::OperandMissing),
        (&Token::Identifier(..), &Token::Identifier(..)) => Presedence::Error(ParseError::OperatorMissing),
        (&Token::Identifier(..), &Token::Mul(..)) => Presedence::Greater,
        (&Token::Identifier(..), &Token::Add(..)) => Presedence::Greater,
        (&Token::Identifier(..), &Token::Cmp(..)) => Presedence::Greater,
        (&Token::Identifier(..), &Token::RightBrace) => Presedence::Greater,
        (&Token::Identifier(..), &Token::End) => Presedence::Greater,

        (&Token::Mul(..), &Token::LeftBrace) => Presedence::Less,
        (&Token::Mul(..), &Token::Identifier(..)) => Presedence::Less,
        (&Token::Mul(..), &Token::Mul(..)) => Presedence::Greater,
        (&Token::Mul(..), &Token::Add(..)) => Presedence::Greater,
        (&Token::Mul(..), &Token::Cmp(..)) => Presedence::Greater,
        (&Token::Mul(..), &Token::RightBrace) => Presedence::Greater,
        (&Token::Mul(..), &Token::End) => Presedence::Greater,

        (&Token::Add(..), &Token::LeftBrace) => Presedence::Less,
        (&Token::Add(..), &Token::Identifier(..)) => Presedence::Less,
        (&Token::Add(..), &Token::Mul(..)) => Presedence::Less,
        (&Token::Add(..), &Token::Add(..)) => Presedence::Greater,
        (&Token::Add(..), &Token::Cmp(..)) => Presedence::Greater,
        (&Token::Add(..), &Token::RightBrace) => Presedence::Greater,
        (&Token::Add(..), &Token::End) => Presedence::Greater,

        (&Token::LeftBrace, &Token::LeftBrace) => Presedence::Less,
        (&Token::LeftBrace, &Token::Identifier(..)) => Presedence::Less,
        (&Token::LeftBrace, &Token::Mul(..)) => Presedence::Less,
        (&Token::LeftBrace, &Token::Add(..)) => Presedence::Less,
        (&Token::LeftBrace, &Token::Cmp(..)) => Presedence::Less,
        (&Token::LeftBrace, &Token::RightBrace) => Presedence::Equal,
        (&Token::LeftBrace, &Token::End) => Presedence::Error(ParseError::UnbalancedLeftBrace),

        (&Token::End, &Token::LeftBrace) => Presedence::Less,
        (&Token::End, &Token::Identifier(..)) => Presedence::Less,
        (&Token::End, &Token::Mul(..)) => Presedence::Less,
        (&Token::End, &Token::Add(..)) => Presedence::Less,
        (&Token::End, &Token::Cmp(..)) => Presedence::Less,
        (&Token::End, &Token::RightBrace) => Presedence::Error(ParseError::UnbalancedRightBrace),
        (&Token::End, &Token::End) => Presedence::Error(ParseError::OperandMissing),

        (&Token::Cmp(..), &Token::LeftBrace) => Presedence::Less,
        (&Token::Cmp(..), &Token::Identifier(..)) => Presedence::Less,
        (&Token::Cmp(..), &Token::Mul(..)) => Presedence::Less,
        (&Token::Cmp(..), &Token::Add(..)) => Presedence::Less,
        (&Token::Cmp(..), &Token::Cmp(..)) => Presedence::Less,
        (&Token::Cmp(..), &Token::RightBrace) => Presedence::Greater,
        (&Token::Cmp(..), &Token::End) => Presedence::Greater
    }
}
