use crate::evaluation::{abs, app, Abstraction, Application, Decode, Encode, Term, Variable};

impl<'a, T: Encode<'a>> Encode<'a> for (T, T) {
    fn encode(&self) -> Term<'a> {
        const SELECTOR: Variable<'_> = Variable::new("s");
        let (a, b) = self;
        abs(SELECTOR, app(app(SELECTOR, a.encode()), b.encode()))
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for (T, T) {
    fn decode(term: &Term<'a>) -> Option<Self> {
        match term {
            Abs! { variable: selector, term: App! { left: App! { left: Term::Var(f), right: a }, right: b } }
                if f == selector =>
            {
                Some((T::decode(a)?, T::decode(b)?))
            }
            _ => None,
        }
    }
}
