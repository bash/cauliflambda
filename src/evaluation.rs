use crate::syntax;
use std::fmt::{self, Write};

#[macro_use]
mod macros;

mod rename;
use fmtastic::Subscript;
pub use rename::*;
mod evaluate;
pub use evaluate::*;
mod free;
pub use free::*;
mod beta;
pub(crate) use beta::*;
mod rename_bound;
pub use rename_bound::*;
mod result;
pub use result::*;
mod substitute;
pub use substitute::*;

pub fn var(name: &str) -> Term<'_> {
    Variable::new(name).into()
}

#[cfg(test)]
pub fn var_with(name: &str, disambiguator: usize) -> Term<'_> {
    Variable::new(name).with_disambiguator(disambiguator).into()
}

pub fn app<'a>(left: impl Into<Term<'a>>, right: impl Into<Term<'a>>) -> Term<'a> {
    Term::App(Box::new(Application::new(left.into(), right.into())))
}

pub fn abs<'a>(variable: impl Into<Variable<'a>>, term: impl Into<Term<'a>>) -> Term<'a> {
    Term::Abs(Box::new(Abstraction::new(variable.into(), term.into())))
}

#[cfg(test)]
fn nested_abs<'a, I, V>(variables: I, term: Term<'a>) -> Term<'a>
where
    V: Into<Variable<'a>>,
    I: IntoIterator<Item = V>,
    I::IntoIter: DoubleEndedIterator,
{
    variables
        .into_iter()
        .rfold(term, |term, v| abs(v.into(), term))
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Step<'a> {
    pub term: Term<'a>,
    pub kind: StepKind,
}

impl<'a> Step<'a> {
    fn new(kind: StepKind, term: Term<'a>) -> Self {
        Step { term, kind }
    }

    fn map(mut self, f: impl FnOnce(Term<'a>) -> Term<'a>) -> Self {
        self.term = f(self.term);
        self
    }

    fn id_or_err(self) -> Result<Term<'a>, Self> {
        if self.kind == StepKind::Id {
            Ok(self.term)
        } else {
            Err(self)
        }
    }

    fn not_id(self) -> Option<Self> {
        (self.kind != StepKind::Id).then_some(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepKind {
    /// No operation was performed.
    Id,
    /// An α-conversion (i.e. renaming of bound variables to avoid conflicts).
    Alpha,
    /// A β-reduction (i.e. substitution of a bound variable with an applied term).
    Beta,
    /// A δ-reduction (in our case this is expansion of definitions).
    Delta,
}

impl fmt::Display for StepKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepKind::Id => Ok(()),
            StepKind::Alpha => f.write_str("α"),
            StepKind::Beta => f.write_str("β"),
            StepKind::Delta => f.write_str("δ"),
        }
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum Term<'a> {
    Var(Variable<'a>),
    Abs(Box<Abstraction<'a>>),
    App(Box<Application<'a>>),
}

impl<'a> From<syntax::Formula<'a>> for Term<'a> {
    fn from(value: syntax::Formula<'a>) -> Self {
        match value {
            syntax::Formula::Abs(abs) => Term::Abs(Box::new((*abs).into())),
            syntax::Formula::App(app) => Term::App(Box::new((*app).into())),
            syntax::Formula::Var(var) => Term::Var(var.into()),
        }
    }
}

impl<'a> fmt::Debug for Term<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(v) => v.fmt(f),
            Term::Abs(a) => a.fmt(f),
            Term::App(a) => a.fmt(f),
        }
    }
}

impl<'a> From<Variable<'a>> for Term<'a> {
    fn from(value: Variable<'a>) -> Self {
        Term::Var(value)
    }
}

impl<'a> From<Abstraction<'a>> for Term<'a> {
    fn from(value: Abstraction<'a>) -> Self {
        Term::Abs(Box::new(value))
    }
}

impl<'a> From<Application<'a>> for Term<'a> {
    fn from(value: Application<'a>) -> Self {
        Term::App(Box::new(value))
    }
}

impl<'a> fmt::Display for Term<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(v) => v.fmt(f),
            Term::Abs(a) => a.fmt(f),
            Term::App(a) => a.fmt(f),
        }
    }
}

/// A free or bound variable. A variable can have a disambiguator that
/// is incremented during evaluation to avoid conflicting names.
///
/// Example: `x`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Variable<'a> {
    pub name: &'a str,
    pub disambiguator: usize,
}

impl<'a> Variable<'a> {
    pub fn new(name: &'a str) -> Self {
        Variable {
            name,
            disambiguator: 0,
        }
    }

    pub fn with_disambiguator(self, disambiguator: usize) -> Self {
        Variable {
            disambiguator,
            ..self
        }
    }
}

impl<'a> From<syntax::Identifier<'a>> for Variable<'a> {
    fn from(value: syntax::Identifier<'a>) -> Self {
        Self::new(value.value)
    }
}

impl<'a> From<&'a str> for Variable<'a> {
    fn from(name: &'a str) -> Self {
        Variable::new(name)
    }
}

impl<'a> From<(&'a str, usize)> for Variable<'a> {
    fn from((name, disambiguator): (&'a str, usize)) -> Self {
        Variable::new(name).with_disambiguator(disambiguator)
    }
}

impl<'a> fmt::Display for Variable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.disambiguator > 0 {
            write!(f, "{}{}", self.name, Subscript(self.disambiguator))
        } else {
            write!(f, "{}", self.name)
        }
    }
}

/// An abstraction (or anonymous function definition).
///
/// Example: `λx.x`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Abstraction<'a> {
    pub variable: Variable<'a>,
    pub term: Term<'a>,
}

impl<'a> Abstraction<'a> {
    pub fn new(variable: Variable<'a>, term: Term<'a>) -> Self {
        Abstraction { variable, term }
    }
}

impl<'a> From<syntax::Abstraction<'a>> for Abstraction<'a> {
    fn from(value: syntax::Abstraction<'a>) -> Self {
        Self::new(value.variable.into(), value.formula.into())
    }
}

impl<'a> fmt::Display for Abstraction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "λ{}.{}", self.variable, self.term)
    }
}

/// A function application where the term on the right is applied to the term on the left.
///
/// Example: `(λx.x) Y`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Application<'a> {
    pub left: Term<'a>,
    pub right: Term<'a>,
}

impl<'a> Application<'a> {
    pub fn new(left: Term<'a>, right: Term<'a>) -> Self {
        Application { left, right }
    }
}

impl<'a> From<syntax::Application<'a>> for Application<'a> {
    fn from(value: syntax::Application<'a>) -> Self {
        Self::new(value.left.into(), value.right.into())
    }
}

impl<'a> fmt::Display for Application<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        with_parenthesis(matches!(self.left, Term::Abs(_)), f, |f| self.left.fmt(f))?;
        f.write_char(' ')?;
        with_parenthesis(matches!(self.right, Term::App(_) | Term::Abs(_)), f, |f| {
            self.right.fmt(f)
        })
    }
}

fn with_parenthesis(
    condition: bool,
    fmt: &mut fmt::Formatter<'_>,
    f: impl FnOnce(&mut fmt::Formatter<'_>) -> fmt::Result,
) -> fmt::Result {
    if condition {
        fmt.write_char('(')?;
    }
    f(fmt)?;
    if condition {
        fmt.write_char(')')?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_adds_necessary_parenthesis() {
        let terms = [
            ("x (y z)", app(var("x"), app(var("y"), var("z")))),
            ("x y z", app(app(var("x"), var("y")), var("z"))),
            ("λx.λy.λz.z", abs("x", abs("y", abs("z", var("z"))))),
            ("λx.x y", abs("x", app(var("x"), var("y")))),
            (
                "X (λx.x y)",
                app(var("X"), abs("x", app(var("x"), var("y")))),
            ),
            ("(λx.x) y", app(abs("x", var("x")), var("y"))),
        ];

        for (syntax, term) in terms {
            assert_eq!(syntax, term.to_string());
        }
    }
}
