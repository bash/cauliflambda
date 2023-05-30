use super::*;
use std::iter;
use StepKind::*;

pub fn evaluate<'a>(term: impl Into<Term<'a>>) -> impl Iterator<Item = Step<'a>> {
    iter::successors(seed_step(term.into()), step).skip(1)
}

fn seed_step(term: Term<'_>) -> Option<Step<'_>> {
    Some(Step::new(Id, term))
}

fn step<'a>(previous: &Step<'a>) -> Option<Step<'a>> {
    reduce(previous.term.clone()).not_id_or(expand).not_id()
}
