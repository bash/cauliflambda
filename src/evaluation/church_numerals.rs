use crate::evaluation::{abs, app, Abstraction, Application, Decode, Encode, Term, Variable};

impl Encode<'static> for u64 {
    fn encode(&self) -> Term<'static> {
        const F: Variable<'_> = Variable::new("f");
        const X: Variable<'_> = Variable::new("x");
        let n = *self;
        abs(F, abs(X, (0..n).fold(Term::Var(X), |expr, _| app(F, expr))))
    }
}

impl Decode<'_> for u64 {
    fn decode(term: &Term<'_>) -> Option<Self> {
        if let Abs! { variable: f, term: Abs! { variable: x, term } } = term {
            decode_church_numeral(term, *f, *x)
        } else {
            None
        }
    }
}

fn decode_church_numeral(mut term: &Term<'_>, f: Variable<'_>, x: Variable<'_>) -> Option<u64> {
    let mut n = 0;
    loop {
        match term {
            App! { left: Term::Var(v), right } if *v == f => {
                n += 1;
                term = right
            }
            Term::Var(v) if *v == x => return Some(n),
            _ => return None,
        }
    }
}
