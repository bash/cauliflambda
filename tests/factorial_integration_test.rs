use cauliflambda::evaluation::evaluate;
use cauliflambda::parse_formula;

#[test]
fn factorial_of_four_can_be_computed() {
    let formula = parse_formula(include_str!("factorial.lc")).unwrap();
    assert!(formula.diagnostics.0.is_empty());
    let last_step = evaluate(formula.value.into()).take(100_000).last().unwrap();
    let normal_form = last_step.term;

    // I don't have alpha equivalence implemented yet, so this will have to do for now.
    assert_eq!("λf.λx.f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f x)))))))))))))))))))))))", normal_form.to_string());
}
