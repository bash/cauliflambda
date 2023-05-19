use std::fmt;
use std::ops::Range;

#[derive(Debug, Clone)]
pub enum Formula<'a> {
    Abs(Box<Abstraction<'a>>),
    App(Box<Application<'a>>),
    Var(Identifier<'a>),
}

impl<'a> Formula<'a> {
    pub fn abs(abstraction: Abstraction<'a>) -> Self {
        Formula::Abs(Box::new(abstraction))
    }

    pub fn app(application: Application<'a>) -> Self {
        Formula::App(Box::new(application))
    }

    pub fn span(&self) -> Span {
        match self {
            Formula::Abs(a) => a.span.clone(),
            Formula::App(a) => a.span.clone(),
            Formula::Var(v) => v.span.clone(),
        }
    }
}

impl<'a> fmt::Display for Formula<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Formula::Abs(abs) => write!(f, "{}", abs),
            Formula::App(app) => write!(f, "{}", app),
            Formula::Var(var) => write!(f, "{}", var),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Abstraction<'a> {
    pub variable: Identifier<'a>,
    pub formula: Formula<'a>,
    pub span: Span,
}

impl<'a> fmt::Display for Abstraction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(λ{}.{})", self.variable, self.formula)
    }
}

#[derive(Debug, Clone)]
pub struct Application<'a> {
    pub left: Formula<'a>,
    pub right: Formula<'a>,
    pub span: Span,
}

impl<'a> fmt::Display for Application<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.left, self.right)
    }
}

#[derive(Debug, Clone)]
pub struct Identifier<'a> {
    pub value: &'a str,
    pub span: Span,
}

impl<'a> fmt::Display for Identifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span(pub Range<usize>);
