use crate::ast::*;
use crate::chars::*;
use std::ops::Range;
use trait_set::trait_set;
use winnow::ascii::{multispace1, not_line_ending};
use winnow::combinator::{alt, cut_err, fold_repeat, repeat};
use winnow::error::VerboseError;
use winnow::sequence::delimited;
use winnow::token::{one_of, tag, take_while};
use winnow::Located;
use winnow::Parser as _;

type Input<'a> = Located<&'a str>;
type IResult<'a, O> = winnow::IResult<Input<'a>, O, VerboseError<Input<'a>>>;

trait_set! {
    trait Parser<'a, O> = winnow::Parser<Input<'a>, O, VerboseError<Input<'a>>>;
}

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
        Some(left) => {
            let span = Range {
                start: left.span().start,
                end: right.span().end,
            };
            Some(Formula::App(Box::new(Application { left, right, span })))
        }
    }
}

fn one_formula(input: Input) -> IResult<Formula> {
    alt((
        scope,
        abstraction.map(Formula::abs),
        identifier.map(Formula::Var),
        scheme.map(Formula::scheme),
    ))
    .parse_next(input)
}

fn lambda(input: Input) -> IResult<char> {
    one_of("&λ").parse_next(input)
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
    repeat(.., alt((comment, discarded(multispace1))))
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
    parenthesized(formula).context("scope").parse_next(input)
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
        .with_span()
        .map(|((_, _, variable, _, _, formula), span)| Abstraction {
            variable,
            formula,
            span,
        })
        .context("abstraction")
        .parse_next(input)
}

fn scheme(input: Input) -> IResult<Scheme> {
    delimited(
        ('[', trivia),
        (identifier, delimited(trivia, symbol, trivia), identifier),
        (trivia, ']'),
    )
    .with_span()
    .map(|((left, symbol, right), span)| Scheme {
        left,
        symbol,
        right,
        span,
    })
    .parse_next(input)
}

fn schematic_definition(input: Input) -> IResult<SchematicDefinition> {
    (scheme, arrow, parenthesized(formula))
        .with_span()
        .map(|((scheme, _, formula), span)| SchematicDefinition {
            scheme,
            formula,
            span,
        })
        .parse_next(input)
}

// TODO: allow unicode symbols
fn symbol(input: Input) -> IResult<Symbol> {
    take_while(1.., is_ascii_symbol)
        .with_span()
        .map(|(value, span)| Symbol { value, span })
        .parse_next(input)
}

fn parenthesized<'a, O>(parser: impl Parser<'a, O>) -> impl Parser<'a, O> {
    delimited(('(', trivia), parser, (trivia, ')'))
}

fn arrow(input: Input) -> IResult<()> {
    discarded(delimited(trivia, tag("->"), trivia)).parse_next(input)
}

fn discarded<'a, O>(parser: impl Parser<'a, O>) -> impl Parser<'a, ()> {
    parser.map(|_| ())
}
