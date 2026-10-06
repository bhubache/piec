use std::ops::Range;

use crate::span::Span;

use ariadne::Label;
use ariadne::Report;
use ariadne::ReportKind;
use ariadne::Source;

// TODO:
// - ErrorCode to map to a particular type of error
// - Level to support errors, warnings, and such
#[derive(Debug)]
pub struct Diagnostic<'a> {
    file_path: &'a str,
    source: &'a str,
    message: String,
    span: Span,
    annotations: Vec<Annotation>,
}

impl<'a> Diagnostic<'a> {
    pub fn new(
        file_path: &'a str,
        source: &'a str,
        message: String,
        span: Span,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            file_path,
            source,
            message,
            span,
            annotations,
        }
    }

    pub fn report(&self) {
        let mut report = Report::build(ReportKind::Error, (self.file_path, Range::from(self.span)))
            .with_message(&self.message);
        for ann in self.annotations.iter() {
            report = report.with_label(
                Label::new((self.file_path, Range::from(ann.span))).with_message(ann.message),
            )
        }
        report
            .finish()
            .eprint((self.file_path, Source::from(self.source)))
            .unwrap();
    }
}

#[derive(Debug)]
pub struct Annotation {
    span: Span,
    message: &'static str,
}

impl Annotation {
    pub fn new(span: Span, message: &'static str) -> Self {
        Self { span, message }
    }
}
