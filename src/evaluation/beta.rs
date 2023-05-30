use super::*;
use EvaluationResult::*;
use Term::*;

pub fn reduce<'a>(term: impl Into<Term<'a>>) -> EvaluationResult<'a> {
    match term.into() {
        App! { left: Abs(abs), right } => {
            Beta(rename_and_substitute(abs.term, abs.variable, &right))
        }
        App! { left, right } => recurse(left, right),
        Abs! { variable, term } => reduce(term).map(|term| abs(variable, term)),
        term => Complete(term),
    }
}

fn recurse<'a>(left: Term<'a>, right: Term<'a>) -> EvaluationResult<'a> {
    match reduce(left) {
        Beta(left) => Beta(app(left, right)),
        Complete(left) => reduce(right).map(|right| app(left, right)),
    }
}

fn rename_and_substitute<'a>(haystack: Term<'a>, needle: Variable, term: &Term<'a>) -> Term<'a> {
    let is_free = is_free_in(term);
    substitute(needle, term, rename_bound(haystack, |v| !is_free(v)).term())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_expressions_that_cannot_be_reduced_futher() {
        let expressions = [λ![x], λ![λa.λb.λc.c], λ![λx.x x (x x)], λ![X(λx.λy.x)]];
        for expression in expressions {
            let result = dbg!(reduce(expression));
            assert!(matches!(result, Complete(_)));
        }
    }

    #[test]
    fn reduces_leftmost_application_first() {
        let expression = λ![ ((λx.x) X) ((λx.x) Y) ];
        let expected = λ![ X ((λx.x) Y) ];
        assert_eq!(expected, reduce(expression).beta().unwrap());
    }

    #[test]
    fn reduces_outermost_application_first() {
        let expression = λ![ (λx.(λy.y) x) X ];
        let expected = λ![ (λy.y) X ];
        assert_eq!(expected, reduce(expression).beta().unwrap());
    }

    #[test]
    fn reduces_leftmost_outermost_application_first() {
        let expression = λ![ ((λx.(λy.y) x) X) ((λx.x) Y) ];
        let expected = λ![ ((λy.y) X) ((λx.x) Y) ];
        assert_eq!(expected, reduce(expression).beta().unwrap());
    }

    #[test]
    fn reduces_application_with_naming_conflict() {
        let expression = λ![ (λy.λx.y) x ];
        let expected = abs(("x", 1), var("x"));
        assert_eq!(expected, reduce(expression).beta().unwrap());
    }
}
