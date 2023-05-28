use super::*;
use EvaluationResult::*;
use Term::*;

pub fn reduce<'a>(term: impl Into<Term<'a>>) -> EvaluationResult<'a> {
    match term.into() {
        App! { left: Abs(abs), right } => {
            Beta(substitute_or_rename_abs(abs.term, abs.variable, &right))
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

fn substitute_or_rename_abs<'a>(haystack: Term<'a>, needle: Variable, term: &Term<'a>) -> Term<'a> {
    let is_free = is_free_in(term);
    substitute(rename_bound(haystack, |v| !is_free(v)), needle, term)
}

fn substitute<'a>(haystack: Term<'a>, needle: Variable, replacement: &Term<'a>) -> Term<'a> {
    match haystack {
        Var(v) if v == needle => replacement.clone(),
        Abs(a) if a.variable == needle => Abs(a),
        Abs! { variable, term } => abs(variable, substitute(term, needle, replacement)),
        App! { left, right } => app(
            substitute(left, needle, replacement),
            substitute(right, needle, replacement),
        ),
        term => term,
    }
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

    fn nested_abs<'a, I, V>(variables: I, term: Term<'a>) -> Term<'a>
    where
        V: Into<Variable<'a>>,
        I: IntoIterator<Item = V>,
        I::IntoIter: DoubleEndedIterator,
    {
        variables
            .into_iter()
            .rfold(term, |term, v| abs(v.into(), term))
    }
}
