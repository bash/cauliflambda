use cauliflambda::Diagnostic;
use cauliflambda::{Diagnostics, DiagnosticsResult};
use codespan_reporting::diagnostic::Diagnostic as ReportedDiagnostic;
use codespan_reporting::diagnostic::Label;
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

pub fn unwrap_diagnostics_result<O>(
    path: &str,
    input: &str,
    result: DiagnosticsResult<O>,
) -> Result<O, ()> {
    match result {
        Ok(output) => {
            print_diagnostics(&path, &input, &output.diagnostics);
            Ok(output.value)
        }
        Err(diagnostics) => {
            print_diagnostics(&path, &input, &diagnostics);
            Err(())
        }
    }
}

fn print_diagnostics(file_name: &str, input: &str, diagnostics: &Diagnostics) {
    let mut files = SimpleFiles::new();
    let file_id = files.add(file_name, input);

    for diagnostic in &diagnostics.0 {
        let diagnostic = to_reported_diagnostic(file_id, diagnostic);
        let writer = StandardStream::stderr(ColorChoice::Always);
        term::emit(&mut writer.lock(), &term_config(), &files, &diagnostic).unwrap();
    }
}

fn term_config() -> term::Config {
    term::Config {
        before_label_lines: 2,
        after_label_lines: 2,
        ..Default::default()
    }
}

fn to_reported_diagnostic(file_id: usize, diagnostic: &Diagnostic) -> ReportedDiagnostic<usize> {
    ReportedDiagnostic::error()
        .with_message(diagnostic.message.as_ref())
        .with_labels(vec![Label::primary(file_id, diagnostic.source.clone())])
}
