use chumsky::{
    extra,
    input::{Stream, ValueInput},
    prelude::{Input, Simple},
    primitive::just,
    span::SimpleSpan,
    Parser,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    True,
    False,
    StmtEnd,
}

fn bool_parser<'a, I: ValueInput<'a, Token = Token, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Token, extra::Err<Simple<'a, Token>>> {
    just(Token::True).or(just(Token::False))
}

fn stmt_parser<'a, I: ValueInput<'a, Token = Token, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Token, extra::Err<Simple<'a, Token>>> {
    bool_parser().then_ignore(just(Token::StmtEnd))
}

fn main() {
    let tokens = [(Token::False, SimpleSpan::new(0, 5))];
    let input_length = 5;
    let token_stream = Stream::from_iter(tokens).spanned((input_length..input_length).into());

    let res = stmt_parser().parse(token_stream).into_result();
    match res {
        Ok(_) => {}
        Err(errs) => {
            for err in errs {
                println!("{}", err);
            }
        }
    }
}
