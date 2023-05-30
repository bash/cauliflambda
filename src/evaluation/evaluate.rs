use super::*;
use std::iter;

pub fn evaluate(term: Term<'_>) -> impl Iterator<Item = Term<'_>> {
    iter::successors(Some(term), |term| reduce(term.clone()).beta()).skip(1)
}
