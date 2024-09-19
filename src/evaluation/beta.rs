use super::*;
use StepKind::*;
use Term::*;
use TermResult::*;

pub(crate) fn reduce<'a>(term: impl Into<Term<'a>>) -> Step<'a> {
    match term.into() {
        App! { left: Abs(abs), right } => rename_and_substitute(abs.term, abs.variable, right),
        App! { left, right } => recurse(left, right),
        Abs! { variable, term } => reduce(term).map(|term| abs(variable, term)),
        term => Step::new(Id, term),
    }
}

fn recurse<'a>(left: Term<'a>, right: Term<'a>) -> Step<'a> {
    match reduce(left).id_or_err() {
        Ok(left) => reduce(right).map(|right| app(left, right)),
        Err(left) => left.map(|left| app(left, right)),
    }
}

fn rename_and_substitute<'a>(haystack: Term<'a>, needle: Variable<'a>, term: Term<'a>) -> Step<'a> {
    match rename_bound(haystack, is_bound_in(&term)) {
        Modified(haystack) => Step::new(Alpha, app(abs(needle, haystack), term)),
        Original(input) => Step::new(Beta, substitute(needle, &term, input)),
    }
}

fn is_bound_in<'a>(term: &'a Term) -> impl Fn(&Variable) -> bool + Clone + 'a {
    let is_free = is_free_in(term);
    move |v| !is_free(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_expressions_that_cannot_be_reduced_futher() {
        let expressions = [λ![x], λ![λa.λb.λc.c], λ![λx.x x (x x)], λ![X(λx.λy.x)]];
        for expression in expressions {
            let result = dbg!(reduce(expression.clone()));
            assert_eq!(StepKind::Id, result.kind);
            assert_eq!(expression, result.term);
        }
    }

    #[test]
    fn reduces_leftmost_application_first() {
        let expression = λ![ ((λx.x) X) ((λx.x) Y) ];
        let expected = λ![ X ((λx.x) Y) ];
        assert_eq!(expected, reduce(expression).term);
    }

    #[test]
    fn reduces_outermost_application_first() {
        let expression = λ![ (λx.(λy.y) x) X ];
        let expected = λ![ (λy.y) X ];
        assert_eq!(expected, reduce(expression).term);
    }

    #[test]
    fn reduces_leftmost_outermost_application_first() {
        let expression = λ![ ((λx.(λy.y) x) X) ((λx.x) Y) ];
        let expected = λ![ ((λy.y) X) ((λx.x) Y) ];
        assert_eq!(expected, reduce(expression).term);
    }

    #[test]
    fn reduces_application_with_naming_conflict() {
        let expression = λ![ (λy.λx.y) x ];
        let expected = abs(("x", 1), var("x"));
        assert_eq!(expected, reduce(reduce(expression).term).term);
    }
}
