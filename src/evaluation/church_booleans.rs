use crate::evaluation::{abs, Abstraction, Decode, Encode, Term, Variable};

impl Encode<'static> for bool {
    fn encode(&self) -> Term<'static> {
        const T: Variable<'_> = Variable::new("t");
        const F: Variable<'_> = Variable::new("f");
        if *self {
            abs(T, abs(F, T))
        } else {
            abs(T, abs(F, F))
        }
    }
}

impl Decode<'_> for bool {
    fn decode(term: &Term<'_>) -> Option<Self> {
        if let Abs! { variable: t, term: Abs! { variable: f, term: Term::Var(value) } } = term {
            if value == t {
                Some(true)
            } else if value == f {
                Some(false)
            } else {
                None
            }
        } else {
            None
        }
    }
}
