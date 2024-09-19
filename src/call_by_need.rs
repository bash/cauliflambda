//! Implementation of the Call-By-Need Lambda Calculus by Z. M. Ariola and M. Felleisen.
//! See: <https://www.cambridge.org/core/services/aop-cambridge-core/content/view/F4FC3C34E9CAE3F4326503E254FCF6F2/S0956796897002724a.pdf/the-call-by-need-lambda-calculus.pdf>
use crate::evaluation::Variable;

/// Expressions `(Λ): M ::= x | λx.M | MM`
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression<'a> {
    Var(Variable<'a>),
    Abs(Box<Abstraction<'a>>),
    App(Box<Application<'a>>),
}

impl<'a> Expression<'a> {
    pub fn value(&self) -> Option<Value<'a>> {
        if let Expression::Abs(abs) = self {
            Some(Value((&**abs).clone()))
        } else {
            None
        }
    }

    pub fn answer(&self) -> Option<Answer<'a>> {
        if let Some(v) = self.value() {
            Some(Answer::Val(v))
        } else if let Expression::App(app) = self {
            if let Expression::Abs(abs) = &app.left {
                if let Some(answer) = abs.expression.answer() {
                    return Some(Answer::App(
                        abs.variable.clone(),
                        Box::new(answer),
                        app.right.clone(),
                    ));
                }
            }
            None
        } else {
            None
        }
    }

    fn evaluation_context(&self, hole: &Expression<'a>) -> Option<EvaluationContext<'a>> {
        if self == hole {
            return Some(EvaluationContext::Hole(self.clone()));
        }

        if let Expression::App(app) = self {
            if let Some(eval) = app.left.evaluation_context(hole) {
                return Some(EvaluationContext::App(Box::new(eval), app.right.clone()));
            }

            if let Expression::Abs(abs) = &app.left {
                if let Some(eval_1) = abs
                    .expression
                    .evaluation_context(&Expression::Var(abs.variable))
                {
                    if let Some(eval_2) = app.right.evaluation_context(hole) {
                        return Some(EvaluationContext::Jit(
                            abs.variable,
                            Box::new(eval_1),
                            Box::new(eval_2),
                        ));
                    }
                }

                // if let Some()
            }
        }

        None
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Abstraction<'a> {
    pub variable: Variable<'a>,
    pub expression: Expression<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Application<'a> {
    pub left: Expression<'a>,
    pub right: Expression<'a>,
}

/// Values, a subset of expressions: `V ::= λx.M`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Value<'a>(Abstraction<'a>);

/// Answers, a subset of expressions: `A ::= V | ((λx.A) M)`
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Answer<'a> {
    /// `λx.M`
    Val(Value<'a>),
    /// `((λx.A) M)`
    App(Variable<'a>, Box<Answer<'a>>, Expression<'a>),
}

/// `(λx.E[x])V = (λx.E[V])V`
fn deref<'a>(expr: Expression<'a>) -> Option<Expression<'a>> {
    todo!()
}

/// `(λx.A)MN = (λx.AN)M`
fn lift<'a>(expr: Expression<'a>) -> Expression<'a> {
    todo!()
}

/// `(λx.E[x])((λy.A)M) = (λy.(λx.E[x])A)M`
fn assoc<'a>(expr: Expression<'a>) -> Expression<'a> {
    todo!()
}

/// Evaluation Contexts:
/// ```
/// E ::= [ ] | EM | (λx.E[x])E | (λx.E)M
/// ```
#[derive(Debug, Clone)]
enum EvaluationContext<'a> {
    /// []
    Hole(Expression<'a>),
    /// EM
    App(Box<EvaluationContext<'a>>, Expression<'a>),
    /// (λx.E[x])E
    Jit(
        Variable<'a>,
        Box<EvaluationContext<'a>>,
        Box<EvaluationContext<'a>>,
    ),
    /// (λx.E)M
    Rec(Variable<'a>, Box<EvaluationContext<'a>>, Expression<'a>),
}
