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

pub(crate) fn church_boolean_for_var<'a>(var: Variable<'a>) -> Option<Term<'a>> {
    match (var.name, var.disambiguator) {
        ("True", 0) => Some(church_boolean(true)),
        ("False", 0) => Some(church_boolean(false)),
        _ => None,
    }
}

pub(crate) fn church_boolean(x: bool) -> Term<'static> {
    if x {
        nested_abs(["a", "b"], var("a"))
    } else {
        nested_abs(["a", "b"], var("b"))
    }
}
