use crate::namefree::*;
use ReduceResult::*;

pub fn reduce_once<'a>(expression: Expression<'a>) -> ReduceResult<'a> {
    accept(expression, &LeftmostOutermostReducer::default())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReduceResult<'a> {
    /// The expression has been reduced once and may
    /// or may not be reduced further.
    Reduced(Expression<'a>),
    /// The expression can't be reduced further
    Complete(Expression<'a>),
}

impl<'a> AsRef<Expression<'a>> for ReduceResult<'a> {
    fn as_ref(&self) -> &Expression<'a> {
        let (ReduceResult::Reduced(e) | ReduceResult::Complete(e)) = self;
        e
    }
}

impl<'a> ReduceResult<'a> {
    fn map(self, f: impl FnOnce(Expression<'a>) -> Expression<'a>) -> ReduceResult<'a> {
        match self {
            ReduceResult::Reduced(e) => ReduceResult::Reduced(f(e)),
            ReduceResult::Complete(e) => ReduceResult::Complete(f(e)),
        }
    }
}

/// Reduces an expression using the "leftmost outermost" aka. "normal order" strategy.
#[derive(Default)]
struct LeftmostOutermostReducer;

impl<'a> Visit<'a> for LeftmostOutermostReducer {
    type Output = ReduceResult<'a>;

    fn constant(&self, constant: Constant<'a>) -> ReduceResult<'a> {
        Complete(constant.into())
    }

    fn variable(&self, variable: Variable) -> ReduceResult<'a> {
        Complete(variable.into())
    }

    fn abstraction(&self, abstraction: Abstraction<'a>) -> ReduceResult<'a> {
        accept(abstraction.0, self).map(abs)
    }

    fn application(&self, application: Application<'a>) -> ReduceResult<'a> {
        if let Expression::Abs(_) = &application.0 {
            todo!()
        } else {
            match accept(application.0, self) {
                Reduced(left) => Reduced(app(left, application.1)),
                Complete(left) => accept(application.1, self).map(|right| app(left, right)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_expressions_that_cannot_be_reduced_futher() {
        let expressions = [
            r#const("foo"),
            abs(abs(abs(var(3)))),
            abs(app(app(var(1), var(1)), app(var(1), var(1)))),
            app(r#const("A"), abs(abs(var(2)))),
        ];
        for expression in expressions {
            let result = dbg!(reduce_once(expression));
            assert!(matches!(result, Complete(_)));
        }
    }
}
