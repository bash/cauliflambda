use std::fmt;

pub fn abs(expr: Expression<'_>) -> Expression<'_> {
    Expression::Abs(Box::new(Abstraction(expr)))
}

pub fn var<'a>(index: usize) -> Expression<'a> {
    Expression::Var(Variable(index))
}

pub fn app<'a>(lhs: Expression<'a>, rhs: Expression<'a>) -> Expression<'a> {
    Expression::App(Box::new(Application(lhs, rhs)))
}

pub fn r#const(name: &str) -> Expression<'_> {
    Expression::Const(Constant(name))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constant<'a>(pub &'a str);

impl<'a> fmt::Display for Constant<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable(pub usize);

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abstraction<'a>(pub Expression<'a>);

impl<'a> fmt::Display for Abstraction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(λ{})", &self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application<'a>(pub Expression<'a>, pub Expression<'a>);

impl<'a> fmt::Display for Application<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.0, self.1)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Expression<'a> {
    Const(Constant<'a>),
    Var(Variable),
    Abs(Box<Abstraction<'a>>),
    App(Box<Application<'a>>),
}

impl<'a> fmt::Debug for Expression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(c) => c.fmt(f),
            Self::Var(v) => v.fmt(f),
            Self::Abs(a) => a.fmt(f),
            Self::App(a) => a.fmt(f),
        }
    }
}

impl<'a> From<Constant<'a>> for Expression<'a> {
    fn from(value: Constant<'a>) -> Self {
        Expression::Const(value)
    }
}

impl<'a> From<Variable> for Expression<'a> {
    fn from(value: Variable) -> Self {
        Expression::Var(value)
    }
}

impl<'a> From<Abstraction<'a>> for Expression<'a> {
    fn from(value: Abstraction<'a>) -> Self {
        Expression::Abs(Box::new(value))
    }
}

impl<'a> From<Application<'a>> for Expression<'a> {
    fn from(value: Application<'a>) -> Self {
        Expression::App(Box::new(value))
    }
}

impl<'a> fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Const(c) => write!(f, "{}", c),
            Expression::Var(v) => write!(f, "{}", v),
            Expression::Abs(a) => write!(f, "{}", a),
            Expression::App(a) => write!(f, "{}", a),
        }
    }
}

pub(crate) trait Visit<'a> {
    type Output;

    fn constant(&self, constant: Constant<'a>) -> Self::Output;

    fn variable(&self, variable: Variable) -> Self::Output;

    fn abstraction(&self, abstraction: Abstraction<'a>) -> Self::Output;

    fn application(&self, application: Application<'a>) -> Self::Output;
}

pub(crate) fn accept<'a, V>(expression: Expression<'a>, visitor: &V) -> V::Output
where
    V: Visit<'a>,
{
    match expression {
        Expression::Const(c) => visitor.constant(c),
        Expression::Var(v) => visitor.variable(v),
        Expression::Abs(a) => visitor.abstraction(*a),
        Expression::App(a) => visitor.application(*a),
    }
}
