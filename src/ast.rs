use std::fmt;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Identifier<'a> {
    pub value: &'a str,
    pub span: Range<usize>,
}

impl<'a> fmt::Display for Identifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Abstraction<'a> {
    pub variable: Identifier<'a>,
    pub formula: Formula<'a>,
    pub span: Range<usize>,
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
    pub span: Range<usize>,
}

impl<'a> fmt::Display for Application<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.left, self.right)
    }
}

/// A symbol used in schematic definitions
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Symbol<'a> {
    pub value: &'a str,
    pub span: Range<usize>,
}

impl<'a> fmt::Display for Symbol<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// A scheme for replacing expressions of a certain form.
/// Example: `[M * N]`
// TODO: Currently schemes are binary only, extend this to allow n-ary schemes (for n >= 1)
#[derive(Debug, Clone)]
pub struct Scheme<'a> {
    pub left: Identifier<'a>,
    pub symbol: Symbol<'a>,
    pub right: Identifier<'a>,
    pub span: Range<usize>,
}

impl<'a> fmt::Display for Scheme<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {} {}]", self.left, self.symbol, self.right)
    }
}

/// A schematic definition.
/// Example: `[M * N] -> (λa.M (N a))`
#[derive(Debug, Clone)]
pub struct SchematicDefinition<'a> {
    pub scheme: Scheme<'a>,
    pub formula: Formula<'a>,
    pub span: Range<usize>,
}

impl<'a> fmt::Display for SchematicDefinition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> ({})", self.scheme, self.formula)
    }
}

#[derive(Debug, Clone)]
pub enum Formula<'a> {
    Abs(Box<Abstraction<'a>>),
    App(Box<Application<'a>>),
    Var(Identifier<'a>),
    Scheme(Box<Scheme<'a>>),
}

impl<'a> Formula<'a> {
    pub fn abs(abstraction: Abstraction<'a>) -> Self {
        Formula::Abs(Box::new(abstraction))
    }

    pub fn app(application: Application<'a>) -> Self {
        Formula::App(Box::new(application))
    }

    pub fn scheme(scheme: Scheme<'a>) -> Self {
        Formula::Scheme(Box::new(scheme))
    }

    pub fn span(&self) -> Range<usize> {
        match self {
            Formula::Abs(a) => a.span.clone(),
            Formula::App(a) => a.span.clone(),
            Formula::Var(v) => v.span.clone(),
            Formula::Scheme(s) => s.span.clone(),
        }
    }
}

impl<'a> fmt::Display for Formula<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Formula::Abs(abs) => write!(f, "{}", abs),
            Formula::App(app) => write!(f, "{}", app),
            Formula::Var(var) => write!(f, "{}", var),
            Formula::Scheme(scheme) => write!(f, "{}", scheme),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Script<'a> {
    pub definitions: Vec<SchematicDefinition<'a>>,
    pub formula: Formula<'a>,
}

impl<'a> fmt::Display for Script<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for definition in &self.definitions {
            writeln!(f, "{definition}")?;
        }
        write!(f, "{}", self.formula)
    }
}
