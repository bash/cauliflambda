use crate::evaluation::{abs, app, Term, Variable};

/// Encodes a value as a λ-[`Term`].
pub trait Encode<'t> {
    fn encode(&self) -> Term<'t>;
}

/// Decodes a value from a λ-[`Term`].
pub trait Decode<'t>: Sized {
    fn decode(term: &Term<'t>) -> Option<Self>;
}

impl<'a> Term<'a> {
    pub fn decode<T: Decode<'a>>(&self) -> Option<T> {
        T::decode(self)
    }
}

/// Encodes an optional value in a similar fashion as church bools
/// as a λ-[`Term`] of the form `λj n.EXPR`.
///
/// See also <https://tau.garden/blog/lc-maybe/>.
impl<'a, T: Encode<'a>> Encode<'a> for Option<T> {
    fn encode(&self) -> Term<'a> {
        const JUST: Variable<'_> = Variable::new("j");
        const NOTHING: Variable<'_> = Variable::new("n");
        match self {
            Some(value) => abs(JUST, abs(NOTHING, app(JUST, value.encode()))),
            None => abs(JUST, abs(NOTHING, NOTHING)),
        }
    }
}

/// Encodes a result in a similar fashion as church bools
/// as a λ-[`Term`] of the form `λj n.EXPR`.
impl<'a, T: Encode<'a>, E: Encode<'a>> Encode<'a> for Result<T, E> {
    fn encode(&self) -> Term<'a> {
        const OK: Variable<'_> = Variable::new("o");
        const ERROR: Variable<'_> = Variable::new("e");
        match self {
            Ok(value) => abs(OK, abs(ERROR, app(OK, value.encode()))),
            Err(error) => abs(OK, abs(ERROR, app(ERROR, error.encode()))),
        }
    }
}
