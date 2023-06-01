use super::*;

#[derive(Default, Clone)]
pub struct Options {
    definitions: Vec<DynamicDefinition>,
}

impl Options {
    pub fn add_church_encoding(self) -> Self {
        self.add_definitions(church_numeral_for_var)
            .add_definitions(church_boolean_for_var)
    }

    pub fn add_definitions(
        mut self,
        f: impl Fn(Variable) -> Option<Term> + Clone + 'static,
    ) -> Self {
        self.definitions.push(DynamicDefinition(Box::new(f)));
        self
    }
}

impl Options {
    pub(crate) fn resolve<'a>(&self, var: Variable<'a>) -> Option<Term<'a>> {
        self.definitions.iter().filter_map(|d| d.0(var)).next()
    }
}

#[derive(Clone)]
struct DynamicDefinition(Box<dyn DynamicDefinitionFn>);

trait DynamicDefinitionFn: Fn(Variable) -> Option<Term> {
    fn clone_box(&self) -> Box<dyn DynamicDefinitionFn>;
}

impl<F> DynamicDefinitionFn for F
where
    F: Fn(Variable) -> Option<Term> + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn DynamicDefinitionFn> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn DynamicDefinitionFn> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
