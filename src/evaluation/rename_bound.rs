use super::*;
use trait_set::trait_set;

trait_set! {
    pub trait RenameBoundPredicate = Fn(&Variable) -> bool + Clone;
}

/// Renames all bound variables in the given term to fit a given predicate.
pub fn rename_bound<'a>(term: Term<'a>, predicate: impl RenameBoundPredicate) -> Term<'a> {
    match term {
        Abs! { variable, term } if !predicate(&variable) => {
            let new_variable = new_variable_for_term(variable, &term, predicate.clone());
            let renamed = rename(variable, new_variable, term);
            abs(new_variable, rename_bound(renamed, predicate))
        }
        App! { left, right } => app(
            rename_bound(left, predicate.clone()),
            rename_bound(right, predicate),
        ),
        term => term,
    }
}

fn new_variable_for_term<'a>(
    variable: Variable<'a>,
    term: &Term<'a>,
    predicate: impl RenameBoundPredicate,
) -> Variable<'a> {
    let is_free = is_free_in(term);
    new_variable(variable, |v| predicate(v) && !is_free(v))
}

fn new_variable(variable: Variable, predicate: impl Fn(&Variable) -> bool) -> Variable {
    (1..)
        .take(if cfg!(test) { 1000 } else { usize::MAX })
        .filter(|d| *d != variable.disambiguator)
        .map(|d| Variable::new(variable.name).with_disambiguator(d))
        .find(predicate)
        .expect("No more disambiguators left, what are you doing?")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_bound_variables_that_satisfy_predicate() {
        let term = abs("x", var("x"));
        assert_eq!(term, rename_bound(term.clone(), |_| true));
    }

    #[test]
    fn ignores_free_variables_that_do_not_satisfy_predicate() {
        let term = var("x");
        assert_eq!(term, rename_bound(term.clone(), |_| false));
    }

    #[test]
    fn renames_bound_variables() {
        for (expected_term, term, taken) in [
            (
                abs(("x", 1), var_with("x", 1)),
                abs("x", var("x")),
                &["x".into()] as &[Variable],
            ),
            (
                abs(("x", 2), var_with("x", 2)),
                abs(("x", 1), var_with("x", 1)),
                &["x".into(), ("x", 1).into()],
            ),
            (
                app(
                    abs(("x", 1), var_with("x", 1)),
                    abs(("x", 1), var_with("x", 1)),
                ),
                app(abs("x", var("x")), abs("x", var("x"))),
                &["x".into()],
            ),
            (
                abs(("x", 1), abs(("y", 1), abs(("z", 1), var("CONST")))),
                abs("x", abs("y", abs("z", var("CONST")))),
                &["x".into(), "y".into(), "z".into()],
            ),
        ] {
            assert_eq!(expected_term, rename_bound(term, |v| !taken.contains(v)));
        }
    }

    #[test]
    fn skips_free_variables_in_abstraction_term_when_renaming() {
        let expected = abs(("x", 2), app(var_with("x", 2), var_with("x", 1)));
        let term = abs("x", app(var("x"), var_with("x", 1)));
        assert_eq!(expected, rename_bound(term, |v| v != &Variable::new("x")));
    }

    #[test]
    #[should_panic(expected = "No more disambiguators left")]
    fn panics_when_all_disambiguators_are_used_up() {
        let term = abs("x", var("x"));
        assert_eq!(term, rename_bound(term.clone(), |_| false));
    }
}
