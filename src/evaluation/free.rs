use super::*;
use std::collections::HashSet;
use std::default::default;
use Term::*;

pub type Variables<'a> = HashSet<Variable<'a>>;

/// Finds the free variables of a given term. Variables are free if they're not bound by an abstraction.
pub fn free_variables<'a>(term: &Term<'a>) -> Variables<'a> {
    let (mut bound, mut free) = default();
    find_free_variables(term, &mut bound, &mut free);
    free
}

pub fn is_free_in<'a>(term: &Term<'a>) -> impl Fn(&Variable) -> bool + Clone + 'a {
    let free = free_variables(term);
    move |variable| free.contains(variable)
}

fn find_free_variables<'a>(term: &Term<'a>, bound: &mut Variables<'a>, free: &mut Variables<'a>) {
    match term {
        Var(variable) => {
            if !bound.contains(variable) {
                free.insert(*variable);
            }
        }
        Abs! { variable, term } => {
            let inserted = bound.insert(*variable);
            find_free_variables(term, bound, free);
            if inserted {
                bound.remove(variable);
            }
        }
        App! { left, right } => {
            find_free_variables(left, bound, free);
            find_free_variables(right, bound, free);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_by_itself_is_free() {
        let variable = Variable::new("x");
        let free = Variables::from([variable]);
        assert_eq!(free, free_variables(&variable.into()));
    }

    #[test]
    fn variable_in_abstraction_is_bound() {
        let variable = Variable::new("x");
        assert!(free_variables(&abs(variable, variable)).is_empty());
    }

    #[test]
    fn variable_in_nested_abstraction_is_bound() {
        let variable = Variable::new("x");
        assert!(free_variables(&abs(variable, app(abs(variable, variable), variable))).is_empty());
    }

    #[test]
    fn variable_in_abstraction_is_free_if_it_has_a_different_name() {
        let bound = Variable::new("x");
        let free = Variable::new("y");
        assert_eq!(
            Variables::from([free]),
            free_variables(&abs(bound, app(bound, free)))
        );
    }

    #[test]
    fn variable_in_application_is_free_according_to_lhs_and_rhs() {
        let bound_1 = Variable::new("b1");
        let bound_2 = Variable::new("b2");
        let free_1 = Variable::new("f1");
        let free_2 = Variable::new("f2");
        let term = app(
            app(abs(bound_1, bound_1), free_1),
            app(abs(bound_2, bound_2), free_2),
        );
        assert_eq!(Variables::from([free_1, free_2]), free_variables(&term));
    }

    #[test]
    fn variable_on_right_side_of_application_is_not_bound() {
        let bound = Variable::new("x");
        let free = Variable::new("y");
        assert_eq!(
            Variables::from([free]),
            free_variables(&app(abs(bound, bound), free))
        );
    }
}
