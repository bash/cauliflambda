use crate::evaluation::{abs, app, Abstraction, Application, Term, Variable};

pub struct Tuple<T>(pub T);

impl<'a, T> From<Tuple<(T, T)>> for Term<'a>
where
    T: Into<Term<'a>>,
{
    fn from(value: Tuple<(T, T)>) -> Self {
        const SELECTOR: Variable<'_> = Variable::new("s");
        let (a, b) = value.0;
        abs(SELECTOR, app(app(SELECTOR, a), b))
    }
}

impl<'a, T> TryFrom<Term<'a>> for Tuple<(T, T)>
where
    T: TryFrom<Term<'a>, Error = ()>,
{
    type Error = ();

    fn try_from(value: Term<'a>) -> Result<Self, Self::Error> {
        match value {
            Abs! { variable: selector, term: App! { left: App! { left: Term::Var(f), right: a }, right: b } }
                if f == selector =>
            {
                Ok(Tuple((a.try_into()?, b.try_into()?)))
            }
            _ => Err(()),
        }
    }
}
