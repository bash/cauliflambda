use crate::evaluation::{abs, app, Abstraction, Application, Term, Variable};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ChurchNumeral(pub u64);

impl From<ChurchNumeral> for Term<'static> {
    fn from(value: ChurchNumeral) -> Self {
        const F: Variable<'_> = Variable::new("f");
        const X: Variable<'_> = Variable::new("x");
        let n = value.0;
        abs(F, abs(X, (0..n).fold(Term::Var(X), |expr, _| app(F, expr))))
    }
}

impl TryFrom<Term<'_>> for ChurchNumeral {
    type Error = ();

    fn try_from(value: Term<'_>) -> Result<Self, Self::Error> {
        if let Abs! { variable: f, term: Abs! { variable: x, term } } = value {
            decode_church_numeral(term, f, x)
                .ok_or(())
                .map(ChurchNumeral)
        } else {
            Err(())
        }
    }
}

fn decode_church_numeral(mut term: Term<'_>, f: Variable<'_>, x: Variable<'_>) -> Option<u64> {
    let mut n = 0;
    loop {
        match term {
            App! { left: Term::Var(v), right } if v == f => {
                n += 1;
                term = right
            }
            Term::Var(v) if v == x => return Some(n),
            _ => return None,
        }
    }
}
