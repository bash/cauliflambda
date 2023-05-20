use crate::diagnostics::*;
use crate::syntax::*;
use trait_set::trait_set;
use unicode_xid::UnicodeXID;
use winnow::ascii::{multispace1, not_line_ending};
use winnow::combinator::{alt, cut_err, fold_repeat, repeat};
use winnow::error::{VerboseError, VerboseErrorKind};
use winnow::sequence::{delimited, preceded};
use winnow::stream::Location;
use winnow::token::{one_of, take_while};
use winnow::{Located, Parser as _};

type Input<'a> = Located<&'a str>;
type Error<'a> = VerboseError<Input<'a>>;
type IResult<'a, O> = winnow::IResult<Input<'a>, O, Error<'a>>;

trait_set! {
    trait Parser<'a, O> = winnow::Parser<Input<'a>, O, VerboseError<Input<'a>>>;
}

pub fn parse_formula(input: &str) -> DiagnosticsResult<Formula<'_>> {
    formula
        .parse(Input::new(input))
        .map(WithDiagnostics::with_empty_diagnostics)
        .map_err(to_diagnostics)
}

fn to_diagnostics(error: Error) -> Diagnostics {
    let (input, error) = error
        .errors
        .first()
        .expect("At least one error was expected");
    let location = input.location();
    Diagnostics(vec![Diagnostic {
        severity: DiagnosticSeverity::Error,
        message: error_to_message(error).into(),
        source: Span {
            start: location,
            end: location,
        },
    }])
}

fn error_to_message(error: &VerboseErrorKind) -> &'static str {
    match error {
        VerboseErrorKind::Context(context) => context,
        VerboseErrorKind::Winnow(_) => "Unexpected input",
    }
}

fn formula(input: Input<'_>) -> IResult<'_, Formula<'_>> {
    fold_repeat(
        1..,
        delimited(trivia, one_formula, trivia),
        || None,
        apply_formula,
    )
    .map(|option| option.unwrap_or_else(|| unreachable!()))
    .parse_next(input)
}

fn apply_formula<'a>(left: Option<Formula<'a>>, right: Formula<'a>) -> Option<Formula<'a>> {
    match left {
        None => Some(right),
        Some(left) => {
            let span = Span::containing(left.span(), right.span());
            Some(Formula::App(Box::new(Application { left, right, span })))
        }
    }
}

fn one_formula(input: Input) -> IResult<Formula> {
    alt((
        parenthesized(formula),
        abstraction.map(Formula::abs),
        identifier.map(Formula::Var),
    ))
    .parse_next(input)
}

fn abstraction(input: Input) -> IResult<Abstraction> {
    preceded(lambda, cut_err((variable_list, preceded('.', formula))))
        .with_span()
        .map(|((variables, formula), span)| create_abstraction(variables, formula, span.into()))
        .parse_next(input)
}

fn create_abstraction<'a>(
    variables: Vec<Identifier<'a>>,
    formula: Formula<'a>,
    span: Span,
) -> Abstraction<'a> {
    let mut variables = variables.into_iter();
    let innermost = Abstraction {
        variable: variables.next_back().unwrap_or_else(|| unreachable!()),
        formula,
        span: span.clone(),
    };
    variables.rfold(innermost, |abs, variable| Abstraction {
        variable,
        formula: Formula::abs(abs),
        span: span.clone(),
    })
}

fn variable_list(input: Input) -> IResult<Vec<Identifier>> {
    repeat(1.., delimited(trivia, identifier, trivia)).parse_next(input)
}

fn lambda(input: Input) -> IResult<char> {
    one_of("&λ\\").parse_next(input)
}

fn identifier(input: Input) -> IResult<Identifier> {
    (
        one_of(is_identifier_start),
        take_while(0.., is_identifier_continue),
    )
        .recognize()
        .with_span()
        .map(|(value, span)| Identifier {
            value,
            span: span.into(),
        })
        .parse_next(input)
}

const LAMBDA: char = 'λ';

fn is_identifier_start(c: char) -> bool {
    (c.is_xid_start() && c != LAMBDA) || c.is_ascii_digit() || matches!(c, '_')
}

fn is_identifier_continue(c: char) -> bool {
    c.is_xid_continue() && c != LAMBDA
}

fn trivia(input: Input) -> IResult<()> {
    repeat(.., alt((comment, discarded(multispace1)))).parse_next(input)
}

fn comment(input: Input) -> IResult<()> {
    discarded(('#', not_line_ending)).parse_next(input)
}

fn parenthesized<'a, O>(parser: impl Parser<'a, O>) -> impl Parser<'a, O> {
    preceded('(', cut_err(delimited(trivia, parser, (trivia, ')'))))
}

fn discarded<'a, O>(parser: impl Parser<'a, O>) -> impl Parser<'a, ()> {
    parser.map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    const IDENTIFIERS: &[&str] = &[
        "a", "A", "lower", "UPPER", "άμδα", "brötli", "0", "1", "42", "foo_bar", "_foo", "_",
    ];

    #[test]
    fn parses_identifiers() {
        for text in IDENTIFIERS {
            let id = parse(text, identifier);
            assert_eq!(*text, id.value);
        }
    }

    const NON_IDENTIFIERS: &[&str] = &["λ", "λλ", "λfoo", "fooλ", "foo-bar", "foo+bar", "-"];

    #[test]
    fn does_not_parse_invalid_identifers() {
        for text in NON_IDENTIFIERS {
            assert!(matches!(identifier.parse(Input::new(text)), Err(_)))
        }
    }

    const ABSTRACTIONS: &[&str] = &["λx.x", "λx.y", "λ x . x", "&x.x", "\\x.x"];

    #[test]
    fn parses_abstraction() {
        for text in ABSTRACTIONS {
            let abs = parse(text, abstraction);
            assert_eq!("x", abs.variable.value);
        }
    }

    #[test]
    fn parses_abstraction_with_more_than_one_variable() {
        let reference = parse("λx.(λy.(λz._))", abstraction);
        assert!(parse("λx y z._", abstraction).syntax_eq(&reference));
    }

    #[test]
    fn trivia_can_be_placed_anywhere_in_formula() {
        let reference = parse("(λx.x) Y", formula);

        let trivias = &[
            "# comment\n",
            "# comment\r\n",
            "\t",
            "   ",
            "\r\n",
            "\n",
            "\r",
        ];
        let inputs = &[
            "{trivia}(λx.x) Y",
            "({trivia}λx.x) Y",
            "(λ{trivia}x.x) Y",
            "(λx{trivia}.x) Y",
            "(λx.{trivia}x) Y",
            "(λx.x{trivia}) Y",
            "(λx.x){trivia} Y",
            "(λx.x) Y {trivia}",
        ];
        for input_template in inputs {
            for trivia in trivias {
                let input = input_template.replace("{trivia}", trivia);
                assert!(parse(dbg!(&input), formula).syntax_eq(&reference));
            }
        }
    }

    #[test]
    fn parses_application() {
        let app = dbg!(parse("A B", formula));
        assert!(
            matches!(&app, Formula::App(a) if matches!(&a.left, Formula::Var(l) if l.value == "A"))
        );
        assert!(
            matches!(&app, Formula::App(a) if matches!(&a.right, Formula::Var(r) if r.value == "B"))
        );
    }

    #[test]
    fn parses_parenthesized_formulas() {
        let inputs = &["VAR", "A B", "λx.x"];
        for input in inputs {
            let parenthesized_input = dbg!(format!("({input})"));
            let reference = parse(input, formula);
            let parenthesized = parse(&parenthesized_input, formula);
            assert!(reference.syntax_eq(&parenthesized));
        }
    }

    #[test]
    fn application_is_left_associative() {
        let reference = dbg!(parse("((((((A B) C) D) E) F) G)", formula));
        let input = "A B C D E F G";
        assert!(parse(input, formula).syntax_eq(&reference));
    }

    #[test]
    fn errors_are_reported_at_correct_location() {
        let inputs = &[
            "!", "A!", "(A!)", "(A)!", "&!", "&x!", "&x.(!)", "&x.!", "&x.x!",
        ];

        for input in inputs {
            test_diagnostic_at_correct_location(input);
        }
    }

    fn test_diagnostic_at_correct_location(input: &str) {
        let error_index = input.find('!').unwrap();
        let expected_span = Span {
            start: error_index,
            end: error_index,
        };

        let diagnostics = parse_formula(input).unwrap_err().0;

        assert_eq!(1, diagnostics.len());
        assert_eq!(expected_span, diagnostics.first().unwrap().source);
    }

    fn parse<'a, O>(input: &'a str, mut parser: impl Parser<'a, O>) -> O {
        parser.parse(Input::new(input)).unwrap()
    }
}
