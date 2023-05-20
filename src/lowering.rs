use crate::namefree::Expression;
use crate::syntax::{Formula, Identifier};
use crate::{namefree as nf, Diagnostic, DiagnosticSeverity, Label};
use crate::{Diagnostics, WithDiagnostics};

/// Lowers a formula to its namefree equivalent.
pub fn lower_formula(formula: Formula<'_>) -> WithDiagnostics<nf::Expression<'_>> {
    let mut context = Context::default();
    let expression = lower(formula, &mut context);
    WithDiagnostics {
        value: expression,
        diagnostics: context.diagnostics,
    }
}

fn lower<'a>(formula: Formula<'a>, context: &mut Context<'a>) -> nf::Expression<'a> {
    analyze(&formula, context);

    match formula {
        Formula::Abs(abstraction) => {
            let context = context.push(abstraction.variable.clone());
            nf::abs(lower(abstraction.formula, context.0))
        }
        Formula::App(application) => nf::app(
            lower(application.left, context),
            lower(application.right, context),
        ),
        Formula::Var(var) => lower_var(var, context),
    }
}

fn lower_var<'a>(variable: Identifier<'a>, scope: &Context<'a>) -> nf::Expression<'a> {
    scope
        .de_brujin_index(&variable)
        .map(nf::var)
        .or_else(|| church_numeral_for_var(&variable))
        .unwrap_or_else(|| nf::r#const(variable.value))
}

fn church_numeral_for_var(variable: &Identifier<'_>) -> Option<Expression<'static>> {
    variable.value.parse().ok().map(church_numeral)
}

fn church_numeral(n: u64) -> Expression<'static> {
    nf::abs(nf::abs(
        (0..n).fold(nf::var(1), |expr, _| nf::app(nf::var(2), expr)),
    ))
}

fn analyze(formula: &Formula, context: &mut Context) {
    if let Formula::Abs(abstraction) = formula {
        if let Formula::App(application) = &abstraction.formula {
            if let Formula::Var(variable) = &application.right {
                if variable.value == abstraction.variable.value {
                    context.emit(
                        Diagnostic::new(DiagnosticSeverity::Warning, "unnecessary abstraction")
                            .with_label(Label::new(abstraction.variable.span.clone()))
                            .with_label(
                                Label::new(variable.span.clone()).with_message(
                                    "help: remove the abstraction and this application",
                                ),
                            ),
                    );
                }
            }
        }
    }
}

#[derive(Default, Debug)]
struct Context<'a> {
    variables: Vec<Identifier<'a>>,
    diagnostics: Diagnostics,
}

impl<'a> Context<'a> {
    fn push<'s>(&'s mut self, identifier: Identifier<'a>) -> VariableGuard<'s, 'a> {
        self.variables.push(identifier);
        VariableGuard(self)
    }

    fn emit(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.0.push(diagnostic);
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
struct VariableGuard<'r, 'a>(&'r mut Context<'a>);

impl<'r, 'a> Drop for VariableGuard<'r, 'a> {
    fn drop(&mut self) {
        self.0.variables.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namefree::*;

    #[test]
    fn lowers_unbound_variable_as_constant() {
        let expr = lower_formula(parse("X")).value;
        assert!(matches!(expr, nf::Expression::Const(c) if c.0 == "X"));
    }

    #[test]
    fn lowers_unbound_natural_numbers_as_church_numerals() {
        let numerals = [
            ("0", abs(abs(var(1)))),
            ("1", abs(abs(app(var(2), var(1))))),
            (
                "5",
                abs(abs(app(
                    var(2),
                    app(var(2), app(var(2), app(var(2), app(var(2), var(1))))),
                ))),
            ),
        ];

        for (input, expected) in numerals {
            let expr = lower_formula(parse(input)).value;
            assert_eq!(expected, expr);
        }
    }

    #[test]
    fn does_not_bound_natural_numbers_as_church_numerals() {
        let inputs = ["λ0.0", "λ1.1", "λ5.5"];
        for input in inputs {
            let expr = lower_formula(parse(input)).value;
            assert_eq!(abs(var(1)), expr);
        }
    }

    #[test]
    fn lowers_application() {
        let expr = lower_formula(parse("A B")).value;
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

    fn parse(input: &str) -> Formula<'_> {
        crate::parsers::parse_formula(input).unwrap().value
    }
}
