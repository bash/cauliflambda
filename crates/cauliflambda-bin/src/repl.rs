use crate::diagnostics::unwrap_diagnostics_result;
use cauliflambda::evaluation::{evaluate_with_side_effects, Decode, Step, Term, Value};
use cauliflambda::parse_program;
use rustyline::error::ReadlineError;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Completer, Editor, Helper, Highlighter, Hinter, Validator};
use std::error::Error;

pub fn repl() -> Result<(), Box<dyn Error>> {
    let mut rl: Editor<ReplHelper, _> = Editor::new()?;
    rl.set_helper(Some(ReplHelper::new()));

    loop {
        match rl.readline(">> ") {
            Ok(input) => {
                rl.add_history_entry(&input)?;
                process_line(&input);
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!("Goodbye ✨");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

#[derive(Helper, Completer, Hinter, Validator, Highlighter)]
struct ReplHelper {
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
}

impl ReplHelper {
    fn new() -> Self {
        ReplHelper {
            validator: MatchingBracketValidator::new(),
        }
    }
}

fn process_line(input: &str) {
    if let Ok(program) = unwrap_diagnostics_result("<stdin>", input, parse_program(input)) {
        let mut count: u64 = 0;
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
    }
}
