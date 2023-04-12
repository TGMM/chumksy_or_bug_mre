use chumsky::{
    extra,
    input::{Stream, ValueInput},
    prelude::{Input, Simple},
    primitive::just,
    span::SimpleSpan,
    Parser,
};

fn bool_parser<'a, I: ValueInput<'a, Token = char, Span = SimpleSpan>>(
) -> impl Parser<'a, I, char, extra::Err<Simple<'a, char>>> {
    just('t').or(just('f'))
}

fn stmt_parser<'a, I: ValueInput<'a, Token = char, Span = SimpleSpan>>(
) -> impl Parser<'a, I, char, extra::Err<Simple<'a, char>>> {
    bool_parser().then_ignore(just(';'))
}

fn parse_str(input: &str) {
    let input_length = input.len();
    let chars = input
        .chars()
        .enumerate()
        .map(|(i, c)| (c, SimpleSpan::new(i, i + 1)));
    let token_stream = Stream::from_iter(chars).spanned((input_length..input_length).into());

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

fn main() {
    // The parser expects either "f;" or "t;"
    // we make it error by intentionally omitting ';'
    println!("Correct error");
    // Prints "Found end of input"
    parse_str("t");
    println!("Incorrect error");
    // Prints "Found 'f'"
    parse_str("f");
}
