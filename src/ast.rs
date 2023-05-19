use std::cmp::min;
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

    pub fn span(&self) -> &Span {
        match self {
            Formula::Abs(a) => &a.span,
            Formula::App(a) => &a.span,
            Formula::Var(v) => &v.span,
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

impl<'a> SyntaxEq for Formula<'a> {
    fn syntax_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Formula::Abs(l), Formula::Abs(r)) => l.syntax_eq(r),
            (Formula::App(l), Formula::App(r)) => l.syntax_eq(r),
            (Formula::Var(l), Formula::Var(r)) => l.syntax_eq(r),
            _ => false,
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

impl<'a> SyntaxEq for Abstraction<'a> {
    fn syntax_eq(&self, other: &Self) -> bool {
        self.variable.syntax_eq(&other.variable) && self.formula.syntax_eq(&other.formula)
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

impl<'a> SyntaxEq for Application<'a> {
    fn syntax_eq(&self, other: &Self) -> bool {
        self.left.syntax_eq(&other.left) && self.right.syntax_eq(&other.right)
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

impl<'a> SyntaxEq for Identifier<'a> {
    fn syntax_eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<Range<usize>> for Span {
    fn from(Range { start, end }: Range<usize>) -> Self {
        Span { start, end }
    }
}

impl Span {
    pub fn containing(a: &Span, b: &Span) -> Span {
        Span {
            start: min(a.start, b.start),
            end: min(a.end, b.end),
        }
    }
}

pub trait SyntaxEq {
    fn syntax_eq(&self, other: &Self) -> bool;
}
