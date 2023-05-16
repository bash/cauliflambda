use self::parser::*;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::{iter, ops::Range};
use winnow::{Located, Parser};

mod ast;
mod parser;

// TODO: multi params
// TODO: understand and use cut_error

fn main() {
    let input = include_str!("../bind.lc");
    let script = script
        .parse(Located::new(input))
        .unwrap_or_else(|error| panic!("{error:#?}"));
    println!("{script}");

    let mut files = SimpleFiles::new();

    let file_id = files.add("example.lc", input);

    let span_1: &Range<usize> = &script.definitions[0].scheme.left.span;
    let span_2: &Range<usize> = &script.definitions[0].scheme.right.span;

    let diagnostic = Diagnostic::note()
        .with_message("Foo bar baz")
        .with_labels(vec![
            Label::primary(file_id, span_1.clone()).with_message("lhs"),
            Label::primary(file_id, span_2.clone()).with_message("rhs"),
        ]);

    // We now set up the writer and configuration, and then finally render the
    // diagnostic to standard error.

    let writer = StandardStream::stderr(ColorChoice::Always);
    let mut config = codespan_reporting::term::Config::default();
    config.before_label_lines = 2;
    config.after_label_lines = 2;

    term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
}
