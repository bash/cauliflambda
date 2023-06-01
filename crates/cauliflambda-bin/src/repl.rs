use crate::diagnostics::unwrap_diagnostics_result;
use cauliflambda::evaluation::{evaluate, Options, Step};
use cauliflambda::parse_formula;
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
    if let Ok(formula) = unwrap_diagnostics_result("<stdin>", input, parse_formula(input)) {
        let mut count = 0;
        for Step { term, kind, .. } in evaluate(formula, Options::default().add_church_numerals()) {
            count += 1;
            println!("->>{kind} {term}");
        }
        println!("Found normal form after {count} steps");
    }
}
