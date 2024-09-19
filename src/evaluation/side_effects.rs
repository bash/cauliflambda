use super::{app, evaluate, Application, Step, StepKind, Term, Variable};
use std::iter;
use trait_set::trait_set;

trait_set! {
    pub trait Perform<'a> = FnMut(Variable<'a>, Term<'a>) -> Option<Term<'a>>;
}

pub fn evaluate_with_side_effects<'a>(
    term: impl Into<Term<'a>>,
    mut perform: impl Perform<'a>,
) -> impl Iterator<Item = Step<'a>> {
    let term = term.into();

    let mut last_term = term.clone();
    let mut inner = evaluate(term);

    iter::from_fn(move || {
        inner
            .next()
            .inspect(|s| last_term = s.term.clone())
            .or_else(|| {
                try_perform(&last_term, &mut perform)
                    .inspect(|t| inner = evaluate(t.clone()))
                    .inspect(|t| last_term = t.clone())
                    .map(|t| Step::new(StepKind::SideEffect, t))
            })
    })
}

fn try_perform<'a>(term: &Term<'a>, mut perform: impl Perform<'a>) -> Option<Term<'a>> {
    match term {
        Term::Var(name) => perform(name.clone(), Term::Var(Variable::new("_"))),
        App! { left: Term::Var(name @ Variable { .. }), right } => {
            perform(name.clone(), right.clone())
        }
        App! { left: left @ Term::App { .. }, right } => {
            try_perform(left, perform).map(|left| app(left, right.clone()))
        }
        _ => None,
    }
}
