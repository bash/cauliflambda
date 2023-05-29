use super::*;
use Term::*;

/// Substitutes a free variable in a term with a replacement term.
/// ### Precondition
/// The input term's bound variables must not conflict with
/// free variables in the replacement term. \
/// Hint: Rename the bound variables using [`rename_bound`] before calling [`substitute`].
pub fn substitute<'a>(needle: Variable, replacement: &Term<'a>, input: Term<'a>) -> Term<'a> {
    match input {
        Var(v) if v == needle => replacement.clone(),
        Var(v) => Var(v),
        Abs(a) if a.variable == needle => Abs(a),
        Abs! { variable, term } => abs(variable, substitute(needle, replacement, term)),
        App! { left, right } => app(
            substitute(needle, replacement, left),
            substitute(needle, replacement, right),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitutes_matching_free_variable() {
        let variable = Variable::new("x");
        let input = variable.into();
        let replacement = var("R");
        assert_eq!(replacement, substitute(variable, &replacement, input));
    }

    #[test]
    fn substitutes_matching_free_variable_in_abstraction() {
        let variable = Variable::new("x");
        let input = abs("y", variable);
        let replacement = var("R");
        assert_eq!(
            abs("y", replacement.clone()),
            substitute(variable, &replacement, input)
        );
    }

    #[test]
    fn substitutes_matching_free_variable_in_application() {
        let variable = Variable::new("x");
        let input = app(variable, variable);
        let replacement = var("R");
        assert_eq!(
            app(replacement.clone(), replacement.clone()),
            substitute(variable, &replacement, input)
        );
    }

    #[test]
    fn does_not_substitutes_non_matching_free_variable() {
        let variable = Variable::new("x");
        let input = var("y");
        let replacement = var("R");
        assert_eq!(input, substitute(variable, &replacement, input.clone()));
    }

    #[test]
    fn does_not_substitute_matching_bound_variable() {
        let variable = Variable::new("x");
        let input = abs(variable, variable);
        let replacement = var("R");
        assert_eq!(input, substitute(variable, &replacement, input.clone()));
    }
}
