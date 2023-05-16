use unicode_xid::UnicodeXID;
use winnow::combinator::alt;
use winnow::stream::Stream;
use winnow::token::{one_of, take_while};
use winnow::{IResult, Parser};

const LAMBDA_CHARACTER: char = 'ƛ';

fn parse_lambda(input: &str) -> IResult<&str, char> {
    one_of("&ƛ").parse_next(input)
}

fn is_xid_start(c: char) -> bool {
    UnicodeXID::is_xid_start(c) && c != LAMBDA_CHARACTER
}

fn is_xid_continue(c: char) -> bool {
    UnicodeXID::is_xid_continue(c) && c != LAMBDA_CHARACTER
}

fn parse_identifier(input: &str) -> IResult<&str, Identifier> {
    let mut parser = (one_of(is_xid_start), take_while(0.., is_xid_continue));
    parser.parse_next(input).map(|(remainder, _)| {
        (
            remainder,
            Identifier(&input[..(input.eof_offset() - remainder.eof_offset())]),
        )
    })
}

#[derive(Debug)]
struct Identifier<'a>(&'a str);

#[derive(Debug)]
struct Abstraction<'a> {
    variable: Identifier<'a>,
    term: Term<'a>,
}

#[derive(Debug)]
struct Substitution<'a> {
    left: Term<'a>,
    right: Term<'a>,
}

#[derive(Debug)]
enum Term<'a> {
    Abs(Box<Abstraction<'a>>),
    Sub(Box<Substitution<'a>>),
    Var(Box<Identifier<'a>>),
}

fn parse_term(input: &str) -> IResult<&str, Term> {
    alt((
        parse_abstraction.map(|a| Term::Abs(Box::new(a))),
        parse_identifier.map(|v| Term::Var(Box::new(v))),
    ))
    .parse_next(input)
}

fn parse_abstraction(input: &str) -> IResult<&str, Abstraction> {
    ('(', parse_lambda, parse_identifier, '.', parse_term, ')')
        .map(|(_, _, variable, _, term, _)| Abstraction { variable, term })
        .parse_next(input)
}

fn main() {
    let abs = parse_term.parse("(ƛx.(ƛy.y))").unwrap();
    println!("{abs:?}");
}
