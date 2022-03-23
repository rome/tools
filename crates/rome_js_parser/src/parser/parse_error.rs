use crate::Parser;
use rome_diagnostics::{Diagnostic, Span};
use rome_js_syntax::{JsSyntaxKind, TextRange};

///! Provides helper functions to build common diagnostic messages

/// Creates a diagnostic saying that the node [name] was expected at range
pub(crate) fn expected_node(name: &str, range: TextRange) -> ExpectedNodeDiagnosticBuilder {
    ExpectedNodeDiagnosticBuilder::with_single_node(name, range)
}

/// Creates a diagnostic saying that any of the nodes in [names] was expected at range
pub(crate) fn expected_any(names: &[&str], range: TextRange) -> ExpectedNodeDiagnosticBuilder {
    ExpectedNodeDiagnosticBuilder::with_any(names, range)
}

#[must_use]
pub(crate) fn expected_token(token: JsSyntaxKind) -> impl ToDiagnostic {
    ExpectedToken(
        token
            .to_string()
            .expect("Expected token to be a punctuation or keyword."),
    )
}

#[must_use]
pub(crate) fn expected_token_any(tokens: &[JsSyntaxKind]) -> impl ToDiagnostic {
    use std::fmt::Write;
    let mut expected = String::new();

    for (index, token) in tokens.iter().enumerate() {
        if index > 0 {
            expected.push_str(", ");
        }

        if index == tokens.len() - 1 {
            expected.push_str("or ");
        }

        let _ = write!(
            &mut expected,
            "'{}'",
            token
                .to_string()
                .expect("Expected token to be a punctuation or keyword.")
        );
    }

    ExpectedTokens(expected)
}

pub(crate) trait ToDiagnostic {
    fn to_diagnostic(self, p: &Parser) -> Diagnostic;
}

impl ToDiagnostic for Diagnostic {
    fn to_diagnostic(self, _: &Parser) -> Diagnostic {
        self
    }
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

impl ToDiagnostic for ExpectedNodeDiagnosticBuilder {
    fn to_diagnostic(self, p: &Parser) -> Diagnostic {
        let range = &self.range;

        let msg = if range.is_empty() && p.tokens.source().get(range.as_range()) == None {
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

struct ExpectedToken(&'static str);

impl ToDiagnostic for ExpectedToken {
    fn to_diagnostic(self, p: &Parser) -> Diagnostic {
        match p.cur() {
            JsSyntaxKind::EOF => p
                .err_builder(&format!("expected `{}` but instead the file ends", self.0))
                .primary(p.cur_range(), "the file ends here"),
            _ => p
                .err_builder(&format!(
                    "expected `{}` but instead found `{}`",
                    self.0,
                    p.cur_src()
                ))
                .primary(p.cur_range(), "unexpected"),
        }
    }
}

struct ExpectedTokens(String);

impl ToDiagnostic for ExpectedTokens {
    fn to_diagnostic(self, p: &Parser) -> Diagnostic {
        match p.cur() {
            JsSyntaxKind::EOF => p
                .err_builder(&format!("expected {} but instead the file ends", self.0))
                .primary(p.cur_range(), "the file ends here"),
            _ => p
                .err_builder(&format!(
                    "expected {} but instead found `{}`",
                    self.0,
                    p.cur_src()
                ))
                .primary(p.cur_range(), "unexpected"),
        }
    }
}
