use super::*;
use TermResult::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TermResult<'a> {
    /// The term was modified (e.g. an alpha conversion or a reduction was performed)
    Modified(Term<'a>),
    /// The original term is returned unchanged.
    Original(Term<'a>),
}

impl<'a> TermResult<'a> {
    pub fn term(self) -> Term<'a> {
        match self {
            Modified(term) => term,
            Original(term) => term,
        }
    }

    pub fn map(self, f: impl FnOnce(Term<'a>) -> Term<'a>) -> Self {
        match self {
            Modified(term) => Modified(f(term)),
            Original(term) => Original(f(term)),
        }
    }

    pub(crate) fn map2(
        left: TermResult<'a>,
        right: TermResult<'a>,
        f: impl FnOnce(Term<'a>, Term<'a>) -> Term<'a>,
    ) -> Self {
        if matches!(left, Modified(_)) || matches!(right, Modified(_)) {
            Modified(f(left.term(), right.term()))
        } else {
            Original(f(left.term(), right.term()))
        }
    }
}
