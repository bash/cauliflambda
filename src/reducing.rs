use crate::namefree::*;
use std::{cmp::Ordering, iter};
use ReduceResult::*;

pub fn reduce_once(expression: Expression<'_>) -> ReduceResult<'_> {
    accept(expression, &LeftmostOutermostReducer::default())
}

pub fn reduce_to_normal_form(expression: Expression<'_>) -> impl Iterator<Item = Expression<'_>> {
    iter::successors(Some(expression), |expression| {
        reduce_once(expression.clone()).reduced()
    })
    .skip(1)
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
        match variable.0.cmp(&self.depth) {
            Ordering::Less => variable.into(),
            Ordering::Equal => insert(self.replacement.clone(), self.depth - 1),
            Ordering::Greater => Variable(variable.0 - 1).into(),
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
            n_abs(3, var(3)),
            abs(app(app(var(1), var(1)), app(var(1), var(1)))),
            app(r#const("A"), abs(abs(var(2)))),
        ];
        for expression in expressions {
            let result = dbg!(reduce_once(expression));
            assert!(matches!(result, Complete(_)));
        }
    }

    #[test]
    fn reduces_outermost_application_first() {
        let expression = app(abs(app(id(), app(r#const("Y"), var(1)))), r#const("X"));

        let reduced = unwrap_reduced(reduce_once(expression));

        let expected = app(id(), app(r#const("Y"), r#const("X")));
        assert_eq!(expected, reduced);
    }

    #[test]
    fn reduces_left_side_of_application_first_if_ambiguous() {
        let left = app(id(), r#const("L"));
        let right = app(id(), r#const("R"));
        let expression = app(left, right.clone());

        let reduced = unwrap_reduced(reduce_once(expression));

        let expected = app(r#const("L"), right);
        assert_eq!(expected, reduced);
    }

    #[test]
    fn reduces_left_side_of_application_first_recursively_if_ambiguous() {
        let left_left = app(id(), r#const("LL"));
        let left_right = app(id(), r#const("LR"));
        let left = app(left_left, left_right.clone());
        let right = app(id(), r#const("R"));
        let expression = app(left, right.clone());

        let reduced = unwrap_reduced(reduce_once(expression));

        let expected = app(app(r#const("LL"), left_right), right);
        assert_eq!(expected, reduced);
    }

    #[test]
    fn adjusts_de_brujin_indexes_for_inserted_expression() {
        let target = n_abs(3, var(3));
        let replacement = n_abs(5, var(6));
        let expression = abs(app(target, replacement));

        let reduced = unwrap_reduced(reduce_once(expression));

        let expected = n_abs(8, var(8));
        assert_eq!(expected, reduced);
    }

    #[test]
    fn adjusts_de_brujin_indexes_for_subsitution_target() {
        let expression = n_abs(3, app(n_abs(2, var(5)), r#const("X")));

        let reduced = unwrap_reduced(reduce_once(expression));

        let expected = n_abs(4, var(4));
        assert_eq!(expected, reduced);
    }

    #[test]
    fn subsitutes_at_all_depths() {
        let expression = app(
            abs(app(var(1), abs(app(var(2), abs(var(3)))))),
            r#const("S"),
        );

        let reduced = unwrap_reduced(reduce_once(expression));

        let expected = app(r#const("S"), abs(app(r#const("S"), abs(r#const("S")))));
        assert_eq!(expected, reduced);
    }

    fn n_abs(abstractions: u64, expression: Expression) -> Expression {
        (0..abstractions).fold(expression, |expr, _| abs(expr))
    }

    fn id() -> Expression<'static> {
        abs(var(1))
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
