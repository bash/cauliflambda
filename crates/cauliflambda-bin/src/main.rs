use cauliflambda::{lower_formula, parse_formula, reduce_to_normal_form};
use diagnostics::unwrap_diagnostics_result;
use repl::repl;
use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;

use crate::diagnostics::print_diagnostics;

mod diagnostics;
mod repl;

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

fn evaluate_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let input = read_to_string(path)?;
    let formula = unwrap_diagnostics_result(&path.to_string_lossy(), &input, parse_formula(&input))
        .unwrap_or_else(|_| exit(1));
    let lowered = lower_formula(formula);
    print_diagnostics(&path.to_string_lossy(), &input, &lowered.diagnostics);

    let mut count = 0;
    for step in reduce_to_normal_form(lowered.value) {
        count += 1;
        println!("->> {step}");
    }
    println!("Found normal form after {count} steps");

    Ok(())
}
