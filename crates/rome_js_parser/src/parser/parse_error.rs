use crate::prelude::*;
use crate::span::Span;
use rome_js_syntax::TextRange;
use std::ops::Range;

///! Provides helper functions to build common diagnostic messages

/// Creates a diagnostic saying that the node [name] was expected at range
pub(crate) fn expected_node(name: &str, range: TextRange) -> ExpectedNodeDiagnosticBuilder {
    ExpectedNodeDiagnosticBuilder::with_single_node(name, range)
}

/// Creates a diagnostic saying that any of the nodes in [names] was expected at range
pub(crate) fn expected_any(names: &[&str], range: TextRange) -> ExpectedNodeDiagnosticBuilder {
    ExpectedNodeDiagnosticBuilder::with_any(names, range)
}

pub(crate) struct ExpectedNodeDiagnosticBuilder {
    names: String,
    range: TextRange,
}

impl ExpectedNodeDiagnosticBuilder {
    fn with_single_node(name: &str, range: TextRange) -> Self {
        ExpectedNodeDiagnosticBuilder {
            names: format!("{} {}", article_for(name), name),
            range,
        }
    }

    fn with_any(names: &[&str], range: TextRange) -> Self {
        debug_assert!(names.len() > 1, "Requires at least 2 names");

        if names.len() < 2 {
            return Self::with_single_node(names.first().unwrap_or(&"<missing>"), range);
        }

        let mut joined_names = String::new();

        for (index, name) in names.iter().enumerate() {
            if index > 0 {
                joined_names.push_str(", ");
            }

            if index == names.len() - 1 {
                joined_names.push_str("or ");
            }

            joined_names.push_str(article_for(name));
            joined_names.push(' ');
            joined_names.push_str(name);
        }

        Self {
            names: joined_names,
            range,
        }
    }
}

impl<P: Parser> ToDiagnostic<P> for ExpectedNodeDiagnosticBuilder {
    fn into_diagnostic(self, p: &P) -> ParseDiagnostic {
        let range = &self.range;

        let msg = if range.is_empty()
            && p.source()
                .text()
                .get(Range::<_>::from(range.as_range()))
                .is_none()
        {
            format!(
                "expected {} but instead found the end of the file",
                self.names
            )
        } else {
            format!(
                "expected {} but instead found '{}'",
                self.names,
                p.text(range.as_range())
            )
        };

        let diag = p.err_builder(&msg, self.range);
        diag.detail(self.range, format!("Expected {} here", self.names))
    }
}

fn article_for(name: &str) -> &'static str {
    match name.chars().next() {
        Some('a' | 'e' | 'i' | 'o' | 'u') => "an",
        _ => "a",
    }
}
