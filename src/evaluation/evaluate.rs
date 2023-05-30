use super::*;
use std::iter;
use StepKind::*;

pub fn evaluate(term: Term<'_>) -> impl Iterator<Item = Step<'_>> {
    iter::successors(seed_step(term), step).skip(1)
}

fn seed_step(term: Term<'_>) -> Option<Step<'_>> {
    Some(Step::new(Id, term))
}

fn step<'a>(previous: &Step<'a>) -> Option<Step<'a>> {
    reduce(previous.term.clone()).not_id()
}
