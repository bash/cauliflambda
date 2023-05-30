use std::borrow::Cow;
use std::ops::Deref;

use crate::syntax::Span;

pub type DiagnosticsResult<T> = Result<WithDiagnostics<T>, Diagnostics>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithDiagnostics<T> {
    pub value: T,
    pub diagnostics: Diagnostics,
}

impl<T> WithDiagnostics<T> {
    pub fn with_empty_diagnostics(value: T) -> Self {
        Self {
            value,
            diagnostics: Diagnostics::default(),
        }
    }
}

impl<T> Deref for WithDiagnostics<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> AsRef<T> for WithDiagnostics<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Diagnostics(pub Vec<Diagnostic>);

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub message: Cow<'static, str>,
    pub labels: Vec<Label>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Label {
    pub location: Span,
    pub message: Option<String>,
}

impl Diagnostic {
    pub(crate) fn new(severity: DiagnosticSeverity, message: impl Into<Cow<'static, str>>) -> Self {
        Diagnostic {
            severity,
            message: message.into(),
            labels: Vec::default(),
        }
    }

    pub(crate) fn with_label(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }
}

impl Label {
    pub(crate) fn new(location: Span) -> Self {
        Label {
            location,
            message: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiagnosticSeverity {
    Warning,
    Error,
}
