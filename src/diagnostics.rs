use std::ops::Range;

use crate::source::SourceFile;
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
    file: &'a SourceFile,
    message: String,
    span: Span,
    annotations: Vec<Annotation>,
}

impl<'a> Diagnostic<'a> {
    pub fn new(
        file: &'a SourceFile,
        message: String,
        span: Span,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            file,
            message,
            span,
            annotations,
        }
    }

    pub fn report(&self) {
        let mut report = Report::build(
            ReportKind::Error,
            (self.file.file_path.clone(), Range::from(self.span)),
        )
        .with_message(&self.message);
        for ann in self.annotations.iter() {
            report = report.with_label(
                Label::new((self.file.file_path.clone(), Range::from(ann.span)))
                    .with_message(ann.message),
            )
        }
        report
            .finish()
            .eprint((
                self.file.file_path.clone(),
                Source::from(self.file.source.clone()),
            ))
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
