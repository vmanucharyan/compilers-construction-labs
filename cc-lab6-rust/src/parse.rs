use presedence::pr;

#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord)]
pub enum MulOp {
    Mul, Div
}

#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord)]
pub enum AddOp {
    Add, Sub
}

#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord)]
pub enum CmpOp {
    LE, L, GE, G, NE
}

#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord)]
pub enum Token {
    Mul(MulOp),
    Add(AddOp),
    Cmp(CmpOp),
    LeftBrace,
    RightBrace,
    Identifier(String),
    End
}

#[derive(Clone, PartialEq, Debug)]
pub enum Presedence { Less, Equal, Greater, Error(ParseError) }

#[derive(Clone, PartialEq, Debug)]
pub enum Symbol {
    Terminal(Token), Nonterminal
}

#[derive(Clone, PartialEq, Debug)]
pub enum ParseError {
    UnknownError(&'static str),
    WrongExpression,
    OperandMissing,
    UnbalancedRightBrace,
    UnbalancedLeftBrace,
    OperatorMissing
}

#[derive(Clone, PartialEq, Debug)]
pub enum ParserState<'a> {
    Advance {
        parse: Vec<i32>,
        input: &'a [Token],
        stack: Vec<Symbol>
    },
    Accept(Vec<i32>),
    Error(ParseError)
}

pub fn parse(input: &[Token]) -> ParserState {
    let initial_state = ParserState::Advance {
        input: input,
        stack: vec!(Symbol::Terminal(Token::End)),
        parse: Vec::new()
    };

    return recur(initial_state);

    fn recur(state: ParserState) -> ParserState {
        println!("{:?}", &state);
        match advance(state) {
            p @ ParserState::Advance{..} => recur(p),
            p @ _ => p
        }
    }
}

fn advance(parser: ParserState) -> ParserState {
    if let ParserState::Advance { input: input, stack: stack, parse: parse } = parser {
        let s = &stack[..];
        match (s, input) {
            ([Symbol::Terminal(Token::End), Symbol::Nonterminal],[Token::End]) => {
                ParserState::Accept(parse)
            },

            ([.., Symbol::Terminal(ref b)], [ref c, ..]) |
            ([.., Symbol::Terminal(ref b), Symbol::Nonterminal], [ref c, ..]) => {
                match pr(b, c) {
                    Presedence::Less | Presedence::Equal => shift(input, &stack, &parse),
                    Presedence::Greater => reduce(input, &stack , &parse),
                    Presedence::Error(e) => ParserState::Error(e)
                }
            },

            _ => panic!("advance(): uncovered pattern...")
        }
    }
    else { parser }
}

fn shift<'a>(input: &'a [Token], stack: &Vec<Symbol>, parse: &Vec<i32>) -> ParserState<'a> {
    ParserState::Advance {
        stack: append(stack, Symbol::Terminal(input[0].clone())),
        parse: parse.clone(),
        input: &input[1..]
    }
}

fn reduce<'a>(input: &'a [Token], stack: &Vec<Symbol>, parse: &Vec<i32>) -> ParserState<'a> {
    // ParserState::Error(ParseError::UnknownError("reduce not implemented"))
    let (pid, new_stack) = match &stack[..] {
        // 6
        [.., Symbol::Terminal(ref b), Symbol::Terminal(ref a)] |
        [.., Symbol::Terminal(ref b), Symbol::Nonterminal, Symbol::Terminal(ref a)]
        if pr(b, a) == Presedence::Less => {
            let new_stack = append_slice(&stack[0..stack.len() - 1], Symbol::Nonterminal);
            (7, new_stack)
        },

        // 1
        [
            x..,
            Symbol::Terminal(ref b),
            Symbol::Nonterminal,
            Symbol::Terminal(ref cmp_op @ Token::Cmp(..)),
            Symbol::Nonterminal
        ]
        if pr(b, cmp_op) == Presedence::Less => {
            let mut new_stack = x.to_vec();
            new_stack.push(Symbol::Terminal(b.clone()));
            new_stack.push(Symbol::Nonterminal);
            (1, new_stack)
        }

        // 4
        [
            x..,
            Symbol::Terminal(ref b),
            Symbol::Nonterminal,
            Symbol::Terminal(ref mul_op @ Token::Mul(..)),
            Symbol::Nonterminal
        ]
        if pr(b, mul_op) == Presedence::Less => {
            let mut new_stack = x.to_vec();
            new_stack.push(Symbol::Terminal(b.clone()));
            new_stack.push(Symbol::Nonterminal);
            (4, new_stack)
        }

        // 2
        [
            x..,
            Symbol::Terminal(ref b),
            Symbol::Nonterminal,
            Symbol::Terminal(ref add_op @ Token::Add(..)),
            Symbol::Nonterminal
        ]
        if pr(b, add_op) == Presedence::Less => {
            let mut new_stack = x.to_vec();
            new_stack.push(Symbol::Terminal(b.clone()));
            new_stack.push(Symbol::Nonterminal);
            (2, new_stack)
        },

        // 6.1
        [
            x..,
            Symbol::Terminal(ref b),
            Symbol::Terminal(Token::LeftBrace),
            Symbol::Nonterminal,
            Symbol::Terminal(Token::RightBrace)
        ]
        if pr(b, &Token::LeftBrace) == Presedence::Less => {
            let mut new_stack = x.to_vec();
            new_stack.push(Symbol::Terminal(b.clone()));
            new_stack.push(Symbol::Nonterminal);
            (6, new_stack)
        },

        // 6.2
        [
            x..,
            Symbol::Terminal(ref b),
            Symbol::Nonterminal,
            Symbol::Terminal(Token::LeftBrace),
            Symbol::Nonterminal,
            Symbol::Terminal(Token::RightBrace)
        ]
        if pr(b, &Token::LeftBrace) == Presedence::Less => {
            let mut new_stack = x.to_vec();
            new_stack.push(Symbol::Terminal(b.clone()));
            new_stack.push(Symbol::Nonterminal);
            new_stack.push(Symbol::Nonterminal);
            (6, new_stack)
        },

        _ => panic!("reduce error")
    };

    ParserState::Advance { input: input, stack: new_stack, parse: append(parse, pid) }
}

fn append<T: Clone>(v: &Vec<T>, e: T) -> Vec<T> {
    let mut new_vec = v.clone();
    new_vec.push(e);
    return new_vec;
}

fn append_slice<T: Clone>(v: &[T], e: T) -> Vec<T> {
    let mut new_vec: Vec<T> = Vec::new();
    for x in v { new_vec.push(x.clone()) };
    new_vec.push(e);
    return new_vec;
}
