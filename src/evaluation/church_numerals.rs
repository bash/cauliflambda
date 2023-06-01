use super::*;

pub(crate) fn church_numeral_for_var<'a>(var: Variable<'a>) -> Option<Term<'a>> {
    (var.disambiguator == 0)
        .then_some(var.name)
        .and_then(|n| n.parse().ok())
        .map(church_numeral)
}

pub(crate) fn church_numeral(n: u64) -> Term<'static> {
    abs(
        "f",
        abs("x", (0..n).fold(var("x"), |term, _| app(var("f"), term))),
    )
}
