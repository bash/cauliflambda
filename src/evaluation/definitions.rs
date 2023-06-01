use super::*;
use StepKind::*;
use Term::*;

/// TODO: Obviously this won't work as we potentially miss a more
/// outer more left beta reduction opportunity. This breaks evaluation for some
/// terms where a normal form exists, but can only be found using leftmost outermost such
/// as when using the y-combinator.
///
/// Precondition: No beta reductions possible.
pub(crate) fn expand<'a, 'o>(term: Term<'a>, options: &'o Options) -> Step<'a> {
    let free = free_variables(&term);
    dbg!(&free);
    expand_with_free(term, options, &free)
}

pub(crate) fn expand_with_free<'o, 'a>(
    term: Term<'a>,
    options: &'o Options,
    free: &Variables,
) -> Step<'a> {
    match term {
        App! { left: Var(var), right } if let Some(term) = resolve(var, options, free) => Step::new(Delta, app(term, right)),
        App! { left, right } => recurse(left, right, options, free),
        Abs! { variable, term } => reduce(term).map(|term| abs(variable, term)),
        Var(var) if let Some(term) = resolve(var, options, free) => Step::new(Delta, term),
        term => Step::new(Id, term),
    }
}

fn recurse<'a>(left: Term<'a>, right: Term<'a>, options: &Options, free: &Variables) -> Step<'a> {
    match expand_with_free(left, options, free).id_or_err() {
        Ok(left) => expand_with_free(right, options, free).map(|right| app(left, right)),
        Err(left) => left.map(|left| app(left, right)),
    }
}

fn resolve<'o, 'a>(var: Variable<'a>, options: &'o Options, free: &Variables) -> Option<Term<'a>> {
    free.contains(&var)
        .then_some(var)
        .and_then(|v| options.resolve(v))
}
