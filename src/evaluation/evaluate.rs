use super::*;
use std::iter;
use StepKind::*;

pub fn evaluate<'a>(term: impl Into<Term<'a>>, options: Options) -> impl Iterator<Item = Step<'a>> {
    iter::successors(seed_step(term.into()), move |p| step(p, &options)).skip(1)
}

fn seed_step(term: Term<'_>) -> Option<Step<'_>> {
    Some(Step::new(Id, term))
}

fn step<'a>(previous: &Step<'a>, options: &Options) -> Option<Step<'a>> {
    reduce(previous.term.clone())
        .not_id_or(|t| expand(t, options))
        .not_id()
}
