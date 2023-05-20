use cauliflambda::namefree::{abs, app, var, Expression};
use cauliflambda::{lower_formula, parse_formula, reduce_to_normal_form};

#[test]
fn factorial_of_four_can_be_computed() {
    let formula = parse_formula(include_str!("factorial.lc")).unwrap();
    assert!(formula.diagnostics.0.is_empty());
    let lowered = lower_formula(formula.value);
    // assert_eq!(lowered.diagnostics.0, Vec::default());
    let normal_form = reduce_to_normal_form(lowered.value)
        .take(100_000)
        .last()
        .unwrap();
    assert_eq!(church_numeral(24), normal_form);
}

fn church_numeral(n: u64) -> Expression<'static> {
    abs(abs((0..n).fold(var(1), |expr, _| app(var(2), expr))))
}
