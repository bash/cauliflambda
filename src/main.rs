use crate::namefree::{reduce_expression, Abstraction, Application, Expression, Variable};

use self::parser::*;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use lowering::lower_to_namefree;
use namefree::Constant;
use std::ops::Range;
use winnow::{Located, Parser};

mod ast;
mod lowering;
mod namefree;
mod parser;

// TODO: multi params
// TODO: understand and use cut_error

fn main() {
    let input = include_str!("../playground.lc");
    let script = script
        .parse(Located::new(input))
        .unwrap_or_else(|error| panic!("{error:#?}"));
    // println!("{script}");
    let lowered = lower_to_namefree(script);
    // println!("{lowered}");

    let reduction_steps = std::iter::successors(Some(lowered), |lowered| {
        let reduced = reduce_expression(lowered.clone());
        if &reduced == lowered {
            None
        } else {
            Some(reduced)
        }
    });

    for step in reduction_steps {
        println!("{step}");
    }

    // let mut files = SimpleFiles::new();

    // let file_id = files.add("example.lc", input);

    // let span_1: &Range<usize> = &script.definitions[0].scheme.left.span;
    // let span_2: &Range<usize> = &script.definitions[0].scheme.right.span;

    // let diagnostic = Diagnostic::note()
    //     .with_message("Foo bar baz")
    //     .with_labels(vec![
    //         Label::primary(file_id, span_1.clone()).with_message("lhs"),
    //         Label::primary(file_id, span_2.clone()).with_message("rhs"),
    //     ]);

    // // We now set up the writer and configuration, and then finally render the
    // // diagnostic to standard error.

    // let writer = StandardStream::stderr(ColorChoice::Always);
    // let mut config = codespan_reporting::term::Config::default();
    // config.before_label_lines = 2;
    // config.after_label_lines = 2;

    // term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();

    // let x = abs(app(
    //     abs(abs(abs(app(var(3), abs(abs(abs(var(6)))))))),
    //     abs(abs(abs(abs(var(5))))),
    // ));
    // println!("{}", x);
    // println!("{}", reduce_expression(x.clone()));
    // println!("{}", reduce_expression(reduce_expression(x)));
}
