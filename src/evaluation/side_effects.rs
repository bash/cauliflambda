use crate::evaluation::{evaluate, Step, Term};

pub fn evaluate_with_side_effects<'a>(term: impl Into<Term<'a>>) -> impl Iterator<Item = Step<'a>> {
    evaluate(term)
}
