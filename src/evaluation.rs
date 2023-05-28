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

pub fn var(name: &str) -> Term<'_> {
    Variable::new(name).into()
}

pub fn app<'a>(left: impl Into<Term<'a>>, right: impl Into<Term<'a>>) -> Term<'a> {
    Term::App(Box::new(Application::new(left.into(), right.into())))
}

pub fn abs<'a>(variable: impl Into<Variable<'a>>, term: impl Into<Term<'a>>) -> Term<'a> {
    Term::Abs(Box::new(Abstraction::new(variable.into(), term.into())))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationResult<'a> {
    /// One beta reduction step has been applied.
    /// The expression may or may not be reduced further.
    Step(EvaluationStepKind, Term<'a>),
    /// The expression can't be reduced further
    Complete(Term<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationStepKind {
    Alpha,
    Beta,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Term<'a> {
    Var(Variable<'a>),
    Abs(Box<Abstraction<'a>>),
    App(Box<Application<'a>>),
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

impl<'a> From<&'a str> for Variable<'a> {
    fn from(name: &'a str) -> Self {
        Variable::new(name)
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

impl<'a> fmt::Display for Application<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        with_parenthesis(matches!(self.left, Term::Abs(_)), f, |f| self.left.fmt(f))?;
        f.write_char(' ')?;
        with_parenthesis(matches!(self.right, Term::App(_)), f, |f| self.right.fmt(f))
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
            ("X λx.x y", app(var("X"), abs("x", app(var("x"), var("y"))))),
            ("(λx.x) y", app(abs("x", var("x")), var("y"))),
        ];

        for (syntax, term) in terms {
            assert_eq!(syntax, term.to_string());
        }
    }
}
