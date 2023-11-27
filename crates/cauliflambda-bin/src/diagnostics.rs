use cauliflambda::{Diagnostic, DiagnosticSeverity};
use cauliflambda::{Diagnostics, DiagnosticsResult};
use codespan_reporting::diagnostic::Label;
use codespan_reporting::diagnostic::{Diagnostic as ReportedDiagnostic, Severity};
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
            print_diagnostics(path, input, &output.diagnostics);
            Ok(output.value)
        }
        Err(diagnostics) => {
            print_diagnostics(path, input, &diagnostics);
            Err(())
        }
    }
}

pub fn print_diagnostics(file_name: &str, input: &str, diagnostics: &Diagnostics) {
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
    ReportedDiagnostic::new(to_severity(diagnostic.severity))
        .with_message(diagnostic.message.as_ref())
        .with_labels(
            diagnostic
                .labels
                .iter()
                .map(|label| {
                    Label::primary(file_id, label.location.clone())
                        .with_message(label.message.to_owned().unwrap_or_default())
                })
                .collect(),
        )
}

fn to_severity(severity: DiagnosticSeverity) -> Severity {
    match severity {
        DiagnosticSeverity::Error => Severity::Error,
        DiagnosticSeverity::Warning => Severity::Warning,
        _ => Severity::Bug,
    }
}
