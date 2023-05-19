use crate::namefree as nf;
use crate::syntax::{Formula, Identifier};

/// Lowers a formula to its namefree equivalent.
pub fn lower_formula<'a>(formula: Formula<'a>) -> nf::Expression<'a> {
    lower(formula, &mut Scope::default())
}

fn lower<'a>(formula: Formula<'a>, scope: &mut Scope<'a>) -> nf::Expression<'a> {
    match formula {
        Formula::Abs(abstraction) => {
            let scope = scope.push(abstraction.variable.clone());
            nf::abs(lower(abstraction.formula, scope.0))
        }
        Formula::App(application) => nf::app(
            lower(application.left, scope),
            lower(application.right, scope),
        ),
        Formula::Var(var) => lower_var(var, &scope),
    }
}

fn lower_var<'a>(variable: Identifier<'a>, scope: &Scope<'a>) -> nf::Expression<'a> {
    scope
        .de_brujin_index(&variable)
        .map(|index| nf::var(index))
        .unwrap_or_else(|| nf::r#const(variable.value))
}

#[derive(Default, Debug)]
struct Scope<'a> {
    variables: Vec<Identifier<'a>>,
}

impl<'a> Scope<'a> {
    fn push<'s>(&'s mut self, identifier: Identifier<'a>) -> VariableGuard<'s, 'a> {
        self.variables.push(identifier);
        VariableGuard(self)
    }

    fn de_brujin_index(&self, identifier: &Identifier<'a>) -> Option<usize> {
        let reverse_index = self
            .variables
            .iter()
            .rposition(|v| v.value == identifier.value)?;
        Some(self.variables.len() - reverse_index)
    }
}

#[derive(Debug)]
struct VariableGuard<'r, 'a>(&'r mut Scope<'a>);

impl<'r, 'a> Drop for VariableGuard<'r, 'a> {
    fn drop(&mut self) {
        self.0.variables.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowers_unbound_variable_as_constant() {
        let expr = lower_formula(parse("X"));
        assert!(matches!(expr, nf::Expression::Const(c) if c.0 == "X"));
    }

    #[test]
    fn lowers_application() {
        let expr = lower_formula(parse("A B"));
        assert!(
            matches!(&expr, nf::Expression::App(a) if matches!(&a.0, nf::Expression::Const(c) if c.0 == "A"))
        );
        assert!(
            matches!(&expr, nf::Expression::App(a) if matches!(&a.1, nf::Expression::Const(c) if c.0 == "B"))
        );
    }

    const ABSTRACTIONS: &[(&str, &str)] = &[
        ("λa.λb.λc. a b c", "(λ(λ(λ((3 2) 1))))"),
        ("λa.a(λb.b(λc.c))", "(λ(1 (λ(1 (λ1)))))"),
        ("λx.x(λx.x(λx.x))", "(λ(1 (λ(1 (λ1)))))"),
        ("λa. (λb. b a) (λc. c a)", "(λ((λ(1 2)) (λ(1 2))))"),
        ("λa. (λb. b a) (λc. b)", "(λ((λ(1 2)) (λb)))"),
    ];

    #[test]
    fn lowers_abstractions() {
        for (input, reference) in ABSTRACTIONS {
            assert_eq!(*reference, dbg!(lower_formula(parse(input)).to_string()));
        }
    }

    fn parse<'a>(input: &'a str) -> Formula<'a> {
        crate::parsers::parse_formula(input).unwrap()
    }
}
