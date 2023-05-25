use super::*;
use EvaluationResult::*;

/// Reduces an expression using the "leftmost outermost" aka. "normal order" strategy.
pub fn evaluate_once(term: Term<'_>) -> EvaluationResult<'_> {
    match term {
        term @ Term::Var(_) => Complete(term),
        term @ Term::Abs(_) => Complete(term),
        Term::App(app) if matches!(app.left, Term::Abs(_)) => todo!(),
        term @ Term::App(_) => Complete(term),
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
            let result = dbg!(evaluate_once(expression));
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
