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
        let expressions = [
            var("foo"),
            nested_abs(["a", "b", "c"], var("c")),
            abs("x", app(app(var("x"), var("x")), app(var("x"), var("x")))),
            app(var("X"), abs("x", abs("y", var("x")))),
        ];
        for expression in expressions {
            let result = dbg!(reduce(expression));
            assert!(matches!(result, Complete(_)));
        }
    }
}
