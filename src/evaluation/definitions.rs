use super::*;
use StepKind::*;
use Term::*;

/// Precondition: No beta reductions possible.
pub(crate) fn expand<'a>(term: Term<'a>) -> Step<'a> {
    match term {
        App! { left: Var(var), right } if let Some(n) = natural_number(var) => Step::new(Delta, app(church_numeral(n), right)),
        App! { left, right } => recurse(left, right),
        Abs! { variable, term } => reduce(term).map(|term| abs(variable, term)),
        Var(var) if let Some(n) = natural_number(var) => Step::new(Delta, church_numeral(n)),
        term => Step::new(Id, term),
    }
}

fn recurse<'a>(left: Term<'a>, right: Term<'a>) -> Step<'a> {
    match expand(left).id_or_err() {
        Ok(left) => expand(right).map(|right| app(left, right)),
        Err(left) => left.map(|left| app(left, right)),
    }
}

fn natural_number(var: Variable) -> Option<u64> {
    (var.disambiguator == 0)
        .then_some(var.name)
        .and_then(|x| x.parse().ok())
}

fn church_numeral(n: u64) -> Term<'static> {
    abs(
        "f",
        abs("x", (0..n).fold(var("x"), |term, _| app(var("f"), term))),
    )
}
