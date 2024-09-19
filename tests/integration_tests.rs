use cauliflambda::evaluation::{evaluate, Value};
use cauliflambda::parse_formula;

#[test]
fn factorial_of_four_can_be_computed() {
    let formula = parse_formula(include_str!("factorial.lc")).unwrap();
    assert!(formula.diagnostics.0.is_empty());
    let last_step = evaluate(formula.value).take(100_000).last().unwrap();
    let normal_form = last_step.term;

    assert_eq!(Value::Integer(24), normal_form.decode().unwrap());
}

#[test]
fn three_is_not_even() {
    let formula = parse_formula(include_str!("is_even.lc")).unwrap();
    assert!(formula.diagnostics.0.is_empty());
    let last_step = evaluate(formula.value).take(100_000).last().unwrap();
    let normal_form = last_step.term;

    assert_eq!(Value::Bool(false), normal_form.decode().unwrap());
}
