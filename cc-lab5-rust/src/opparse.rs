#[derive(Clone, PartialEq, Show)]
pub enum MulOp {
    Mul, Div
}

#[derive(Clone, PartialEq, Show)]
pub enum AddOp {
    Add, Sub
}

#[derive(Clone, PartialEq, Show)]
pub enum CmpOp {
    LE, L, GE, G, E
}

#[derive(Clone, PartialEq, Show)]
pub enum Token {
    Mul(MulOp),
    Add(AddOp),
    Cmp(CmpOp),
    LeftBrace,
    RightBrace,
    Identifier,
    End
}

#[derive(Clone, PartialEq, Show)]
enum Presedence { Less, Equal, Greater, Error(ParseError) }

#[derive(Clone, PartialEq, Show)]
pub enum Symbol {
    Terminal(Token), Nonterminal
}

#[derive(Clone, PartialEq, Show)]
pub enum ParseError {
    UnknownError(&'static str),
    OperandMissing,
    UnbalancedRightBrace,
    UnbalancedLeftBrace,
    OperatorMissing
}

#[derive(Clone, PartialEq, Show)]
pub enum ParserState {
    Advance {
        parse: Vec<i32>,
        input: Vec<Token>,
        stack: Vec<Symbol>
    },
    Accept(Vec<i32>),
    Error(ParseError)
}

impl ParserState {
    pub fn new(input: &[Token]) -> ParserState {
        ParserState::Advance {
            input: {
                let mut i = input.to_vec();
                i.push(Token::End);
                i
            },
            stack: vec!(Symbol::Terminal(Token::End)),
            parse: Vec::new()
        }
    }

    pub fn parse(self) -> ParserState {
        println!("{:?}", &self);
        match self {
            ParserState::Advance { input: i, stack: s, parse: p } => {
                ParserState::advance(i, s, p).parse()
            },
            ParserState::Accept(..) => self,
            ParserState::Error(..) => self
        }
    }

    fn advance(input: Vec<Token>, stack: Vec<Symbol>, parse: Vec<i32>) -> ParserState {
        match (&stack[..], &input[..]) {
            ([Symbol::Terminal(Token::End), Symbol::Nonterminal], [Token::End]) => {
                ParserState::Accept(parse)
            },
            ([.., Symbol::Terminal(ref b)], [ref c,..]) => {
                match pr(b, c) {
                    Presedence::Less | Presedence::Equal => ParserState::shift(b, &stack[..], &input[..], parse),
                    Presedence::Greater => ParserState::reduce(&stack[..], &input[..], parse),
                    Presedence::Error(e) => ParserState::Error(e)
                }
            },
            _ => ParserState::Error(ParseError::UnknownError("Nonterminal on top of the stack"))
        }
    }

    fn shift(b: &Token, stack: &[Symbol], input: &[Token], parse: Vec<i32>) -> ParserState {
        let mut  new_stack = stack.to_vec();
        new_stack.push(Symbol::Terminal(input[0].clone()));
        ParserState::Advance { input: input[1..].to_vec(), stack: new_stack, parse: parse }
    }

    fn reduce(stack: &[Symbol], input: &[Token], parse: Vec<i32>) -> ParserState {
        let (new_stack, pid) = match stack {
            [
                 Symbol::Terminal(ref b),
                 gamma..,
                 Symbol::Terminal(ref a)
            ]
            if pr(b, a) == Presedence::Less => {
                println!("reduce: 6");
                let mut new_stack = vec!(Symbol::Terminal(b.clone()));
                for v in gamma { new_stack.push(v.clone()); }
                new_stack.push(Symbol::Nonterminal);
                (new_stack, 6)
            },

            [
                Symbol::Terminal(ref b), Symbol::Nonterminal,
                Symbol::Terminal(ref mul @ Token::Mul(..)),
                Symbol::Nonterminal
            ]
            if pr(b, mul) == Presedence::Less => {
                let new_stack = vec!(
                    Symbol::Terminal(b.clone()),
                    Symbol::Nonterminal, Symbol::Terminal(mul.clone()), Symbol::Nonterminal
                );
                (new_stack, 3)
            },

            [
                Symbol::Terminal(ref b),
                Symbol::Nonterminal,
                Symbol::Terminal(ref add @ Token::Add(..)),
                Symbol::Nonterminal
            ]
            if pr(b, add) == Presedence::Less => {
                let new_stack = vec!(
                    Symbol::Terminal(b.clone()),
                    Symbol::Nonterminal, Symbol::Terminal(add.clone()), Symbol::Nonterminal
                );
                (new_stack, 1)
            },

            [
                Symbol::Terminal(ref b),
                Symbol::Nonterminal,
                gamma..,
                Symbol::Terminal(Token::LeftBrace),
                Symbol::Nonterminal,
                Symbol::Terminal(Token::RightBrace)
            ]
            if pr(b, &Token::LeftBrace) == Presedence::Less => {
                let mut new_stack = vec!(Symbol::Terminal(b.clone()));
                for v in gamma { new_stack.push(v.clone()); }
                new_stack.push(Symbol::Nonterminal);
                (new_stack, 5)
            },

            _ => panic!("reduce(): uncovered pattern")
        };

        let mut new_parse = parse.clone();
        new_parse.push(pid);

        ParserState::Advance { input: input.to_vec(), stack: new_stack, parse: new_parse}
    }
}
