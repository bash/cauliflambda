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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'a> {
    Const(Constant<'a>),
    Var(Variable),
    Abs(Box<Abstraction<'a>>),
    App(Box<Application<'a>>),
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
