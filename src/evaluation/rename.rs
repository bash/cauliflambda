use super::*;

/// Renames a free variable in the given term.
pub fn rename<'a>(old: Variable<'a>, new: Variable<'a>, term: Term<'a>) -> Term<'a> {
    match term {
        Term::Var(var) if var == old => new.into(),
        term @ Term::Var(_) => term,
        Term::Abs(abs) if abs.variable != old => {
            Abstraction::new(abs.variable, rename(old, new, abs.term)).into()
        }
        term @ Term::Abs(_) => term,
        Term::App(app) => {
            Application::new(rename(old, new, app.left), rename(old, new, app.right)).into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renames_matching_variable() {
        let old = Variable::new("x");
        let new = old.with_disambiguator(1);
        assert_eq!(Term::Var(new), rename(old, new, old.into()));
    }

    #[test]
    fn does_not_rename_non_matching_variable() {
        let old = Variable::new("x");
        let new = old.with_disambiguator(1);
        let term = var("y");
        assert_eq!(term.clone(), rename(old, new, term));
    }

    #[test]
    fn renames_matching_variables_in_application() {
        let old = Variable::new("x");
        let new = old.with_disambiguator(1);
        let term = app(old, old);
        assert_eq!(app(new, new), rename(old, new, term));
    }

    #[test]
    fn renames_matching_free_variables() {
        let old = Variable::new("x");
        let new = old.with_disambiguator(1);
        let bound = Variable::new("y");
        let term = abs(bound, old);
        assert_eq!(abs(bound, new), rename(old, new, term));
    }

    #[test]
    fn does_not_rename_matching_bound_variables() {
        let old = Variable::new("x");
        let new = old.with_disambiguator(1);
        let term = abs(old, old);
        assert_eq!(term.clone(), rename(old, new, term));
    }
}
