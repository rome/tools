use crate::Parser;
use rslint_errors::{Diagnostic, Span};
use rslint_syntax::JsSyntaxKind;
use std::ops::Range;

///! Provides helper functions to build common diagnostic messages

/// Creates a diagnostic saying that the node [name] was expected at range
pub(crate) fn expected_node(name: &str, range: Range<usize>) -> ExpectedNodeDiagnosticBuilder {
    ExpectedNodeDiagnosticBuilder::with_single_node(name, range)
}

/// Creates a diagnostic saying that any of the nodes in [names] was expected at range
pub(crate) fn expected_any(names: &[&str], range: Range<usize>) -> ExpectedNodeDiagnosticBuilder {
    ExpectedNodeDiagnosticBuilder::with_any(names, range)
}

#[must_use]
pub(crate) fn expected_token(token: JsSyntaxKind) -> impl ToDiagnostic {
    ExpectedToken(token)
}

pub trait ToDiagnostic {
    fn to_diagnostic(self, p: &Parser) -> Diagnostic;
}

impl ToDiagnostic for Diagnostic {
    fn to_diagnostic(self, _: &Parser) -> Diagnostic {
        self
    }
}

struct ExpectedToken(JsSyntaxKind);

pub(crate) struct ExpectedNodeDiagnosticBuilder {
    names: String,
    range: Range<usize>,
}

impl ToDiagnostic for ExpectedToken {
    fn to_diagnostic(self, p: &Parser) -> Diagnostic {
        let kind = self.0;

        match p.cur() {
            JsSyntaxKind::EOF => p
                .err_builder(&format!(
                    "expected `{}` but instead the file ends",
                    kind.to_string()
                        .map(|x| x.to_string())
                        .unwrap_or_else(|| format!("{:?}", kind))
                ))
                .primary(p.cur_tok().range(), "the file ends here"),
            _ => p
                .err_builder(&format!(
                    "expected `{}` but instead found `{}`",
                    kind.to_string()
                        .map(|x| x.to_string())
                        .unwrap_or_else(|| format!("{:?}", kind)),
                    p.cur_src()
                ))
                .primary(p.cur_tok().range(), "unexpected"),
        }
    }
}

impl ExpectedNodeDiagnosticBuilder {
    fn with_single_node(name: &str, range: Range<usize>) -> Self {
        ExpectedNodeDiagnosticBuilder {
            names: format!("{} {}", article_for(name), name),
            range,
        }
    }

    fn with_any(names: &[&str], range: Range<usize>) -> Self {
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

impl ToDiagnostic for ExpectedNodeDiagnosticBuilder {
    fn to_diagnostic(self, p: &Parser) -> Diagnostic {
        let range = &self.range;

        let msg = if range.is_empty() && p.tokens.source().get(range.to_owned()) == None {
            format!(
                "expected {} but instead found the end of the file",
                self.names
            )
        } else {
            format!(
                "expected {} but instead found '{}'",
                self.names,
                p.source(range.as_text_range())
            )
        };

        let diag = p.err_builder(&msg);
        diag.primary(&self.range, format!("Expected {} here", self.names))
    }
}

fn article_for(name: &str) -> &'static str {
    match name.chars().next() {
        Some('a' | 'e' | 'i' | 'o' | 'u') => "an",
        _ => "a",
    }
}
