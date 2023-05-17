use crate::ast::{Formula, Identifier};
use crate::namefree as nf;

pub fn lower_to_namefree<'a>(formula: Formula<'a>) -> nf::Expression<'a> {
    lower(formula, &mut Scope::default())
}

fn lower<'a>(formula: Formula<'a>, scope: &mut Scope<'a>) -> nf::Expression<'a> {
    match formula {
        Formula::Abs(abstraction) => scope.track(abstraction.variable.clone(), move |scope| {
            nf::abs(lower(abstraction.formula, scope))
        }),
        Formula::App(application) => nf::app(
            lower(application.left, scope),
            lower(application.right, scope),
        ),
        Formula::Var(var) => lower_var(*var, &scope),
        Formula::Scheme(_) => todo!(),
    }
}

fn lower_var<'a>(variable: Identifier<'a>, scope: &Scope<'a>) -> nf::Expression<'a> {
    scope
        .de_brujin_index(&variable)
        .map(|index| nf::var(index))
        .unwrap_or_else(|| nf::r#const(variable.value))
}

#[derive(Default)]
struct Scope<'a>(Vec<Identifier<'a>>);

impl<'a> Scope<'a> {
    fn de_brujin_index(&self, identifier: &Identifier<'a>) -> Option<usize> {
        let reverse_index = self.0.iter().rposition(|v| v.value == identifier.value)?;
        Some(self.0.len() - reverse_index)
    }

    fn track<O>(&mut self, identifier: Identifier<'a>, block: impl FnOnce(&mut Self) -> O) -> O {
        self.0.push(identifier);
        let output = block(self);
        self.0.pop();
        output
    }
}
