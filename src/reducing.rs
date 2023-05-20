use crate::namefree::*;
use ReduceResult::*;

pub fn reduce_once(expression: Expression<'_>) -> ReduceResult<'_> {
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

impl<'a> ReduceResult<'a> {
    fn reduced(self) -> Option<Expression<'a>> {
        if let Reduced(expression) = self {
            Some(expression)
        } else {
            None
        }
    }
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
        if let Expression::Abs(abstraction) = application.0 {
            Reduced(substitute(&application.1, abstraction.0))
        } else {
            match accept(application.0, self) {
                Reduced(left) => Reduced(app(left, application.1)),
                Complete(left) => accept(application.1, self).map(|right| app(left, right)),
            }
        }
    }
}

fn substitute<'a>(replacement: &Expression<'a>, target: Expression<'a>) -> Expression<'a> {
    accept(
        target,
        &Substituter {
            replacement,
            depth: 1,
        },
    )
}

struct Substituter<'r, 'a> {
    replacement: &'r Expression<'a>,
    depth: usize,
}

impl<'r, 'a> Substituter<'r, 'a> {
    fn descend(&self) -> Self {
        Self {
            depth: self.depth + 1,
            ..*self
        }
    }
}

impl<'r, 'a> Visit<'a> for Substituter<'r, 'a> {
    type Output = Expression<'a>;

    fn constant(&self, constant: Constant<'a>) -> Self::Output {
        constant.into()
    }

    fn variable(&self, variable: Variable) -> Self::Output {
        if variable.0 == self.depth {
            insert(self.replacement.clone(), self.depth - 1)
        } else if variable.0 > self.depth {
            Variable(variable.0 - 1).into()
        } else {
            variable.into()
        }
    }

    fn abstraction(&self, abstraction: Abstraction<'a>) -> Self::Output {
        Abstraction(accept(abstraction.0, &self.descend())).into()
    }

    fn application(&self, application: Application<'a>) -> Self::Output {
        Application(accept(application.0, self), accept(application.1, self)).into()
    }
}

fn insert(expression: Expression, insertion_depth: usize) -> Expression {
    accept(
        expression,
        &Inserter {
            insertion_depth,
            depth: 0,
        },
    )
}

struct Inserter {
    insertion_depth: usize,
    depth: usize,
}

impl Inserter {
    fn descend(&self) -> Self {
        Self {
            depth: self.depth + 1,
            ..*self
        }
    }
}

impl<'a> Visit<'a> for Inserter {
    type Output = Expression<'a>;

    fn constant(&self, constant: Constant<'a>) -> Self::Output {
        constant.into()
    }

    fn variable(&self, variable: Variable) -> Self::Output {
        if variable.0 > self.depth {
            Variable(variable.0 + self.insertion_depth).into()
        } else {
            variable.into()
        }
    }

    fn abstraction(&self, abstraction: Abstraction<'a>) -> Self::Output {
        Abstraction(accept(abstraction.0, &self.descend())).into()
    }

    fn application(&self, application: Application<'a>) -> Self::Output {
        Application(accept(application.0, self), accept(application.1, self)).into()
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

    #[test]
    fn reduces_outermost_application_first() {}

    #[test]
    fn reduces_left_side_of_application_first_if_ambiguous() {}

    #[test]
    fn reduces_left_side_of_application_first_recursively_if_ambiguous() {}

    #[test]
    fn adjusts_de_brujin_indexes_for_inserted_expression() {}

    #[test]
    fn adjusts_de_brujin_indexes_for_subsitution_target() {
        let expression = abs(abs(abs(app(abs(abs(var(5))), r#const("X")))));
        let expected = abs(abs(abs(abs(var(4)))));
        let reduced = unwrap_reduced(reduce_once(expression));
        assert_eq!(expected, reduced);
    }

    fn unwrap_reduced(result: ReduceResult) -> Expression {
        match result {
            Reduced(expression) => expression,
            Complete(_) => {
                panic!("Expected a reduced expression, but the expression is not reducible")
            }
        }
    }
}