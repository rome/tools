use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

use crate::line_index::{LineCol, LineIndex};
use anyhow::{Context, Result};
use rome_analyze::ActionCategory;
use rome_console::fmt::Termcolor;
use rome_console::fmt::{self, Formatter};
use rome_console::MarkupBuf;
use rome_diagnostics::termcolor::NoColor;
use rome_diagnostics::Severity;
use rome_diagnostics::{Applicability, Diagnostic, SuggestionChange};
use rome_rowan::{TextRange, TextSize};
use rome_service::workspace::CodeAction;
use tower_lsp::jsonrpc::Error as LspError;
use tower_lsp::lsp_types::{self as lsp};
use tracing::error;

pub(crate) fn position(line_index: &LineIndex, offset: TextSize) -> lsp::Position {
    let line_col = line_index.line_col(offset);
    lsp::Position::new(line_col.line, line_col.col)
}

pub(crate) fn range(line_index: &LineIndex, range: TextRange) -> lsp::Range {
    let start = position(line_index, range.start());
    let end = position(line_index, range.end());
    lsp::Range::new(start, end)
}

pub(crate) fn offset(line_index: &LineIndex, position: lsp::Position) -> Result<TextSize> {
    let line_col = LineCol {
        line: position.line as u32,
        col: position.character as u32,
    };

    line_index
        .offset(line_col)
        .with_context(|| format!("position {position:?} is out of range"))
}

pub(crate) fn text_range(line_index: &LineIndex, range: lsp::Range) -> Result<TextRange> {
    let start = offset(line_index, range.start)?;
    let end = offset(line_index, range.end)?;
    Ok(TextRange::new(start, end))
}

pub(crate) fn code_fix_to_lsp(
    url: &lsp::Url,
    line_index: &LineIndex,
    diagnostics: &[lsp::Diagnostic],
    action: CodeAction,
) -> lsp::CodeAction {
    // Mark diagnostics emitted by the same rule as resolved by this action
    let diagnostics: Vec<_> = if matches!(action.category, ActionCategory::QuickFix) {
        diagnostics
            .iter()
            .filter_map(|d| {
                let code = d.code.as_ref().and_then(|code| match code {
                    lsp::NumberOrString::String(code) => Some(code.as_str()),
                    lsp::NumberOrString::Number(_) => None,
                })?;

                if code == action.rule_name {
                    Some(d.clone())
                } else {
                    None
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    let kind = match action.category {
        ActionCategory::QuickFix => Some(lsp::CodeActionKind::QUICKFIX),
        ActionCategory::Refactor => Some(lsp::CodeActionKind::REFACTOR),
    };

    let suggestion = action.suggestion;

    let mut changes = HashMap::new();
    let edits = match suggestion.substitution {
        SuggestionChange::String(new_text) => {
            vec![lsp::TextEdit {
                range: range(line_index, suggestion.span.range),
                new_text,
            }]
        }
        SuggestionChange::Indels(indels) => indels
            .into_iter()
            .map(|indel| lsp::TextEdit {
                range: range(line_index, indel.delete),
                new_text: indel.insert,
            })
            .collect(),
    };

    changes.insert(url.clone(), edits);

    let edit = lsp::WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    };

    lsp::CodeAction {
        title: print_markup(&suggestion.msg),
        kind,
        diagnostics: if !diagnostics.is_empty() {
            Some(diagnostics)
        } else {
            None
        },
        edit: Some(edit),
        command: None,
        is_preferred: if matches!(suggestion.applicability, Applicability::Always) {
            Some(true)
        } else {
            None
        },
        disabled: None,
        data: None,
    }
}

/// Convert an [rome_diagnostics::Diagnostic] to a [lsp::Diagnostic], using the span
/// of the diagnostic's primary label as the diagnostic range.
/// Requires a [LineIndex] to convert a byte offset range to the line/col range
/// expected by LSP.
pub(crate) fn diagnostic_to_lsp(
    diagnostic: Diagnostic,
    url: &lsp::Url,
    line_index: &LineIndex,
) -> Option<lsp::Diagnostic> {
    let primary = diagnostic.primary?;

    let related_information = if !diagnostic.children.is_empty() {
        Some(
            diagnostic
                .children
                .into_iter()
                .map(|label| lsp::DiagnosticRelatedInformation {
                    location: lsp::Location {
                        uri: url.clone(),
                        range: range(line_index, label.span.range),
                    },

                    message: print_markup(&label.msg),
                })
                .collect(),
        )
    } else {
        None
    };
    Some(lsp::Diagnostic::new(
        range(line_index, primary.span.range),
        Some(match diagnostic.severity {
            Severity::Help => lsp::DiagnosticSeverity::HINT,
            Severity::Note => lsp::DiagnosticSeverity::INFORMATION,
            Severity::Warning => lsp::DiagnosticSeverity::WARNING,
            Severity::Error | Severity::Bug => lsp::DiagnosticSeverity::ERROR,
        }),
        diagnostic
            .code
            .map(|code| lsp::NumberOrString::String(code.name().into())),
        Some("rome".into()),
        diagnostic
            .summary
            .unwrap_or_else(|| print_markup(&diagnostic.title)),
        related_information,
        diagnostic.tag.map(|tag| {
            let mut result = Vec::new();

            if tag.is_unnecessary() {
                result.push(lsp::DiagnosticTag::UNNECESSARY);
            }

            if tag.is_deprecated() {
                result.push(lsp::DiagnosticTag::DEPRECATED);
            }

            result
        }),
    ))
}

/// Convert a piece of markup into a String
fn print_markup(markup: &MarkupBuf) -> String {
    let mut message = Termcolor(NoColor::new(Vec::new()));
    fmt::Display::fmt(markup, &mut Formatter::new(&mut message))
        // SAFETY: Writing to a memory buffer should never fail
        .unwrap();

    // SAFETY: Printing uncolored markup never generates non UTF-8 byte sequences
    String::from_utf8(message.0.into_inner()).unwrap()
}

/// Helper to create a [tower_lsp::jsonrpc::Error] from a message
pub(crate) fn into_lsp_error(msg: impl Display + Debug) -> LspError {
    let mut error = LspError::internal_error();
    error!("Error: {}", msg);
    error.message = msg.to_string();
    error.data = Some(format!("{msg:?}").into());
    error
}

pub(crate) fn panic_to_lsp_error(err: Box<dyn Any + Send>) -> LspError {
    let mut error = LspError::internal_error();

    match err.downcast::<String>() {
        Ok(msg) => {
            error.message = *msg;
        }
        Err(err) => match err.downcast::<&str>() {
            Ok(msg) => {
                error.message = msg.to_string();
            }
            Err(_) => {
                error.message = String::from("Rome encountered an unknown error");
            }
        },
    }

    error
}
