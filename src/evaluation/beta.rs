use super::*;
use std::default::default;
use StepKind::*;
use Term::*;
use TermResult::*;

pub(crate) fn reduce<'a>(term: impl Into<Term<'a>>, options: &Options) -> Step<'a> {
    let mut ctx = Context {
        options,
        bound: default(),
    };
    reduce_with_context(term.into(), &mut ctx)
}

struct Context<'a, 'o> {
    options: &'o Options,
    bound: Vec<Variable<'a>>,
}

fn reduce_with_context<'a>(term: Term<'a>, ctx: &mut Context<'a, '_>) -> Step<'a> {
    match term {
        App! { left: Abs(abs), right } => rename_and_substitute(abs.term, abs.variable, right),
        App! { left: Var(var), right } if let Some(term) = resolve(var, ctx) => Step::new(Delta, app(term, right)),
        App! { left, right } => recurse_app(left, right, ctx),
        Abs! { variable, term } => recurse_abs(variable, term, ctx),
        Var(var) if let Some(term) = resolve(var, ctx) => Step::new(Delta, term),
        term => Step::new(Id, term),
    }
}

fn recurse_abs<'a>(var: Variable<'a>, term: Term<'a>, ctx: &mut Context<'a, '_>) -> Step<'a> {
    ctx.bound.push(var);
    let result = reduce_with_context(term, ctx).map(|term| abs(var, term));
    ctx.bound.pop();
    result
}

fn recurse_app<'a>(left: Term<'a>, right: Term<'a>, ctx: &mut Context<'a, '_>) -> Step<'a> {
    match reduce_with_context(left, ctx).id_or_err() {
        Ok(left) => reduce_with_context(right, ctx).map(|right| app(left, right)),
        Err(left) => left.map(|left| app(left, right)),
    }
}

fn rename_and_substitute<'a>(haystack: Term<'a>, needle: Variable<'a>, term: Term<'a>) -> Step<'a> {
    match rename_bound(haystack, is_not_free_in(&term)) {
        Modified(haystack) => Step::new(Alpha, app(abs(needle, haystack), term)),
        Original(input) => Step::new(Beta, substitute(needle, &term, input)),
    }
}

fn is_not_free_in<'a>(term: &'a Term) -> impl Fn(&Variable) -> bool + Clone + 'a {
    let free = free_variables(term);
    move |variable| !free.contains(variable)
}

fn resolve<'a>(var: Variable<'a>, ctx: &Context<'a, '_>) -> Option<Term<'a>> {
    (!ctx.bound.contains(&var))
        .then_some(var)
        .and_then(|v| ctx.options.resolve(v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_expressions_that_cannot_be_reduced_futher() {
        let expressions = [λ![x], λ![λa.λb.λc.c], λ![λx.x x (x x)], λ![X(λx.λy.x)]];
        for expression in expressions {
            let result = dbg!(reduce(expression.clone()));
            assert_eq!(StepKind::Id, result.kind);
            assert_eq!(expression, result.term);
        }
    }

    #[test]
    fn reduces_leftmost_application_first() {
        let expression = λ![ ((λx.x) X) ((λx.x) Y) ];
        let expected = λ![ X ((λx.x) Y) ];
        assert_eq!(expected, reduce(expression).term);
    }

    #[test]
    fn reduces_outermost_application_first() {
        let expression = λ![ (λx.(λy.y) x) X ];
        let expected = λ![ (λy.y) X ];
        assert_eq!(expected, reduce(expression).term);
    }

    #[test]
    fn reduces_leftmost_outermost_application_first() {
        let expression = λ![ ((λx.(λy.y) x) X) ((λx.x) Y) ];
        let expected = λ![ ((λy.y) X) ((λx.x) Y) ];
        assert_eq!(expected, reduce(expression).term);
    }

    #[test]
    fn reduces_application_with_naming_conflict() {
        let expression = λ![ (λy.λx.y) x ];
        let expected = abs(("x", 1), var("x"));
        assert_eq!(expected, reduce(reduce(expression).term).term);
    }

    fn reduce<'a>(term: impl Into<Term<'a>>) -> Step<'a> {
        super::reduce(term, &Options::default())
    }
}
