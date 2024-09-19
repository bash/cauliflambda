use cauliflambda::evaluation::{evaluate_with_side_effects, Decode as _, Step, Term, Value};
use cauliflambda::parse_program;
use diagnostics::unwrap_diagnostics_result;
use repl::repl;
use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;

mod diagnostics;
mod repl;
mod side_effects;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args_os().skip(1).collect::<Vec<_>>();
    match args.len() {
        0 => repl(),
        1 => evaluate_file(Path::new(&args[0])),
        _ => help(),
    }
}

fn help() -> Result<(), Box<dyn Error>> {
    println!("Usage: {} [FILE]", env::args().next().unwrap());
    Ok(())
}

// TODO: print normal form to stdout, everything else to stderr
fn evaluate_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let input = read_to_string(path)?;
    let program = unwrap_diagnostics_result(&path.to_string_lossy(), &input, parse_program(&input))
        .unwrap_or_else(|_| exit(1));

    println!("{}", program.formula);

    let mut count = 0;
    let mut normal_form: Term = program.formula.clone().into();
    for Step { term, kind, .. } in evaluate_with_side_effects(program.formula) {
        count += 1;
        normal_form = term.clone();
        println!("->>{kind} {term}");
    }
    if let Some(value) = Value::decode(&normal_form) {
        println!("~~> {value}");
    }
    println!("Found normal form after {count} steps");

    Ok(())
}
