use core::fmt;

pub fn abs<'a>(expr: Expression<'a>) -> Expression<'a> {
    Expression::Abs(Box::new(Abstraction(expr)))
}

pub fn var<'a>(index: usize) -> Expression<'a> {
    Expression::Var(Variable(index))
}

pub fn app<'a>(lhs: Expression<'a>, rhs: Expression<'a>) -> Expression<'a> {
    Expression::App(Box::new(Application(lhs, rhs)))
}

pub fn r#const<'a>(name: &'a str) -> Expression<'a> {
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
        write!(f, "λ{}", &self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application<'a>(pub Expression<'a>, pub Expression<'a>);

impl<'a> fmt::Display for Application<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})({})", &self.0, &self.1)
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

fn accept<'a, O>(expression: Expression<'a>, visitor: &impl Visit<'a, O>) -> O {
    match expression {
        Expression::Const(c) => visitor.constant(c),
        Expression::Var(v) => visitor.variable(v),
        Expression::Abs(a) => visitor.abstraction(*a),
        Expression::App(a) => visitor.application(*a),
    }
}

trait Visit<'a, O> {
    fn constant(&self, constant: Constant<'a>) -> O;

    fn variable(&self, variable: Variable) -> O;

    fn abstraction(&self, abstraction: Abstraction<'a>) -> O;

    fn application(&self, application: Application<'a>) -> O;
}

struct Reducer;

impl<'a> Visit<'a, Expression<'a>> for Reducer {
    fn constant(&self, constant: Constant<'a>) -> Expression<'a> {
        Expression::Const(constant)
    }

    fn variable(&self, variable: Variable) -> Expression<'a> {
        Expression::Var(variable)
    }

    fn abstraction(&self, abstraction: Abstraction<'a>) -> Expression<'a> {
        Expression::Abs(Box::new(Abstraction(accept(abstraction.0, self))))
    }

    fn application(&self, application: Application<'a>) -> Expression<'a> {
        match application.0 {
            Expression::Abs(abs) => accept(
                abs.0,
                &Substituter {
                    expression: &application.1,
                    depth: 1,
                },
            ),
            _ => Expression::App(Box::new(Application(
                accept(application.0, self),
                accept(application.1, self),
            ))),
        }
    }
}

// Substitues variables with a given expression
struct Substituter<'e, 'a> {
    expression: &'e Expression<'a>,
    depth: usize,
}

impl<'e, 'a> Visit<'a, Expression<'a>> for Substituter<'e, 'a> {
    fn constant(&self, constant: Constant<'a>) -> Expression<'a> {
        Expression::Const(constant)
    }

    fn variable(&self, variable: Variable) -> Expression<'a> {
        if variable.0 == self.depth {
            accept(
                self.expression.clone(),
                &VariableDepthAdjuster {
                    insertion_depth: self.depth - 1,
                    depth: 0,
                },
            )
        } else {
            Expression::Var(variable)
        }
    }

    fn abstraction(&self, abstraction: Abstraction<'a>) -> Expression<'a> {
        let substituter = Substituter {
            depth: self.depth + 1,
            ..*self
        };
        Expression::Abs(Box::new(Abstraction(accept(abstraction.0, &substituter))))
    }

    fn application(&self, application: Application<'a>) -> Expression<'a> {
        Expression::App(Box::new(Application(
            accept(application.0, self),
            accept(application.1, self),
        )))
    }
}

struct VariableDepthAdjuster {
    insertion_depth: usize,
    depth: usize,
}

impl<'a> Visit<'a, Expression<'a>> for VariableDepthAdjuster {
    fn constant(&self, constant: Constant<'a>) -> Expression<'a> {
        Expression::Const(constant)
    }

    fn variable(&self, variable: Variable) -> Expression<'a> {
        if variable.0 > self.depth {
            Expression::Var(Variable(variable.0 + self.insertion_depth))
        } else {
            Expression::Var(variable)
        }
    }

    fn abstraction(&self, abstraction: Abstraction<'a>) -> Expression<'a> {
        let adjuster = VariableDepthAdjuster {
            depth: self.depth + 1,
            ..*self
        };
        Expression::Abs(Box::new(Abstraction(accept(abstraction.0, &adjuster))))
    }

    fn application(&self, application: Application<'a>) -> Expression<'a> {
        Expression::App(Box::new(Application(
            accept(application.0, self),
            accept(application.1, self),
        )))
    }
}

pub fn reduce_expression(expression: Expression) -> Expression {
    accept(expression, &Reducer)
}
