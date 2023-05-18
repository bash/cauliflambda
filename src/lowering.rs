use std::collections::HashMap;

use crate::ast::{
    abs, Abstraction, Formula, Identifier, SchematicDefinition, Scheme, Script, Symbol,
};
use crate::namefree::{self as nf};

pub fn lower_to_namefree<'a>(script: Script<'a>) -> nf::Expression<'a> {
    let definitions = collect_definitions(script.definitions);
    let mut scope = Scope::with_definitions(definitions);
    lower(script.formula, &mut scope)
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
        Formula::Scheme(scheme) => todo!(),
    }
}

fn lower_var<'a>(variable: Identifier<'a>, scope: &Scope<'a>) -> nf::Expression<'a> {
    scope
        .de_brujin_index(&variable)
        .map(|index| nf::var(index))
        .unwrap_or_else(|| nf::r#const(variable.value))
}

// fn lower_scheme<'a>(scheme: Scheme<'a>, scope: &mut Scope<'a>) -> nf::Expression<'a> {
//     let definition = &scope.definitions[&scheme.symbol]; // TODO: proper error
//     let abs = lower(
//         abs(
//             definition.scheme.left.clone(),
//             abs(definition.scheme.right.clone(), definition.formula.clone()),
//         ),
//         &mut scope.definitions_only(),
//     );
//     // let app = nf::app(abs, lower(scheme.left));

//     todo!()

//     // todo!()
//     // let definition_as_abs = lower(Formula::Abs(Abstraction {
//     //     variable: definition.scheme.left,
//     // }), scope);
//     // app(app(lhs, scheme.left), scheme.right)
// }

#[derive(Default)]
struct Scope<'a> {
    variables: Vec<Identifier<'a>>,
    definitions: HashMap<Symbol<'a>, SchematicDefinition<'a>>,
}

impl<'a> Scope<'a> {
    fn with_definitions(definitions: HashMap<Symbol<'a>, SchematicDefinition<'a>>) -> Self {
        Self {
            definitions,
            ..Default::default()
        }
    }

    fn definitions_only(&self) -> Self {
        Self::with_definitions(self.definitions.clone())
    }

    fn de_brujin_index(&self, identifier: &Identifier<'a>) -> Option<usize> {
        let reverse_index = self
            .variables
            .iter()
            .rposition(|v| v.value == identifier.value)?;
        Some(self.variables.len() - reverse_index)
    }

    fn track<O>(&mut self, identifier: Identifier<'a>, block: impl FnOnce(&mut Self) -> O) -> O {
        self.variables.push(identifier);
        let output = block(self);
        self.variables.pop();
        output
    }
}

fn collect_definitions<'a>(
    definitions: Vec<SchematicDefinition<'a>>,
) -> HashMap<Symbol<'a>, SchematicDefinition<'a>> {
    // TODO: handle duplicate defs
    definitions
        .into_iter()
        .map(|d| (d.scheme.symbol.clone(), d))
        .collect()
}
