use crate::syntax::Span;

pub type DiagnosticsResult<T> = Result<WithDiagnostics<T>, Diagnostics>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithDiagnostics<T>(T, Diagnostics);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostics(pub Vec<Diagnostic>);

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiagnosticSeverity {
    Error,
    Warning,
}
