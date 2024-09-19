use cauliflambda::evaluation::{evaluate_with_side_effects, Encode, Step};
use cauliflambda::parse_program;
use diagnostics::unwrap_diagnostics_result;
use repl::repl;
use side_effects::perform_side_effect;
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
    for Step { term, kind, .. } in evaluate_with_side_effects(program.formula, perform_side_effect)
    {
        count += 1;
        println!("->>{kind} {term}");
    }
    println!("Found normal form after {count} steps");

    Ok(())
}
