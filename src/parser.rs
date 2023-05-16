use crate::ast::*;
use unicode_xid::UnicodeXID;
use winnow::ascii::{multispace1, not_line_ending};
use winnow::combinator::{alt, cut_err, fold_repeat, repeat};
use winnow::error::VerboseError;
use winnow::sequence::delimited;
use winnow::token::{one_of, tag, take_while};
use winnow::{Located, Parser};

type Input<'a> = Located<&'a str>;
type IResult<'a, O> = winnow::IResult<Input<'a>, O, VerboseError<Input<'a>>>;

pub fn formula(input: Input) -> IResult<Formula> {
    fold_repeat(
        1..,
        delimited(trivia, one_formula, trivia),
        || None,
        formula_folder,
    )
    .context("term")
    .map(|option| option.unwrap())
    .parse_next(input)
}

pub fn script(input: Input) -> IResult<Script> {
    (
        repeat(.., delimited(trivia, schematic_definition, trivia)),
        formula,
    )
        .map(|(definitions, formula)| Script {
            definitions,
            formula,
        })
        .parse_next(input)
}

fn formula_folder<'a>(left: Option<Formula<'a>>, right: Formula<'a>) -> Option<Formula<'a>> {
    match left {
        None => Some(right),
        Some(left) => Some(Formula::App(Box::new(Application { left, right }))),
    }
}

fn one_formula(input: Input) -> IResult<Formula> {
    alt((
        scope,
        abstraction.map(|a| Formula::Abs(Box::new(a))),
        identifier.map(|v| Formula::Var(Box::new(v))),
        scheme.map(|s| Formula::Scheme(Box::new(s))),
    ))
    .parse_next(input)
}

const LAMBDA: char = 'λ';

fn lambda(input: Input) -> IResult<char> {
    one_of("&λ").parse_next(input)
}

fn is_xid_start(c: char) -> bool {
    c.is_xid_start() && c != LAMBDA || matches!(c, '0'..='9')
}

fn is_xid_continue(c: char) -> bool {
    c.is_xid_continue() && c != LAMBDA
}

fn identifier(input: Input) -> IResult<Identifier> {
    let parser = (one_of(is_xid_start), take_while(0.., is_xid_continue));
    parser
        .context("identifier")
        .recognize()
        .with_span()
        .map(|(value, span)| Identifier { value, span })
        .parse_next(input)
}

fn trivia(input: Input) -> IResult<()> {
    repeat(.., alt((comment, multispace1.map(|_| ()))))
        .context("trivia")
        .parse_next(input)
}

fn comment(input: Input) -> IResult<()> {
    ('#', not_line_ending)
        .context("comment")
        .map(|_| ())
        .parse_next(input)
}

fn scope(input: Input) -> IResult<Formula> {
    delimited('(', cut_err(formula), ')')
        .context("scope")
        .parse_next(input)
}

fn abstraction(input: Input) -> IResult<Abstraction> {
    (
        lambda,
        trivia,
        cut_err(identifier),
        trivia,
        '.',
        cut_err(formula),
    )
        .map(|(_, _, variable, _, _, formula)| Abstraction { variable, formula })
        .context("abstraction")
        .parse_next(input)
}

fn scheme(input: Input) -> IResult<Scheme> {
    delimited(
        ('[', trivia),
        (identifier, delimited(trivia, symbol, trivia), identifier),
        (trivia, ']'),
    )
    .map(|(left, symbol, right)| Scheme {
        left,
        symbol,
        right,
    })
    .parse_next(input)
}

fn schematic_definition(input: Input) -> IResult<SchematicDefinition> {
    (
        scheme,
        delimited((trivia, tag("->"), trivia, '(', trivia), formula, ')'),
    )
        .map(|(scheme, formula)| SchematicDefinition { scheme, formula })
        .parse_next(input)
}

// ascSymbol from https://www.haskell.org/onlinereport/haskell2010/haskellch2.html#x7-160002.2
// with the exception of & and . as we use these characters in our grammar.
fn is_ascii_symbol(c: char) -> bool {
    matches!(
        c,
        '!' | '#'
            | '$'
            | '%'
            | '⋆'
            | '+'
            | '/'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | '\\'
            | '^'
            | '|'
            | '-'
            | '~'
            | ':'
    )
}

// TODO: allow unicode symbols
fn symbol(input: Input) -> IResult<Symbol> {
    take_while(1.., is_ascii_symbol)
        .map(Symbol)
        .parse_next(input)
}
