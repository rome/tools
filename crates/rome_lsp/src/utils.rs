use std::collections::HashMap;
use std::fmt::Display;

use crate::line_index::{LineCol, LineIndex};
use rome_analyze::ActionCategories;
use rome_analyze::AnalyzerAction;
use rome_analyze::AnalyzerDiagnostic;
use rome_console::fmt::Termcolor;
use rome_console::fmt::{self, Formatter};
use rome_console::MarkupBuf;
use rome_diagnostics::termcolor::NoColor;
use rome_diagnostics::Severity;
use rome_rowan::AstNode;
use tower_lsp::jsonrpc::Error as LspError;
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::{self, Diagnostic};
use tracing::error;

use rome_js_syntax::{TextRange, TextSize};

pub(crate) fn position(line_index: &LineIndex, offset: TextSize) -> lsp_types::Position {
    let line_col = line_index.line_col(offset);
    lsp_types::Position::new(line_col.line, line_col.col)
}

pub(crate) fn range(line_index: &LineIndex, range: TextRange) -> lsp_types::Range {
    let start = position(line_index, range.start());
    let end = position(line_index, range.end());
    lsp_types::Range::new(start, end)
}

pub(crate) fn offset(line_index: &LineIndex, position: lsp_types::Position) -> TextSize {
    let line_col = LineCol {
        line: position.line as u32,
        col: position.character as u32,
    };
    line_index.offset(line_col)
}

pub(crate) fn text_range(line_index: &LineIndex, range: lsp_types::Range) -> TextRange {
    let start = offset(line_index, range.start);
    let end = offset(line_index, range.end);
    TextRange::new(start, end)
}

pub(crate) fn code_fix_to_lsp(
    url: &lsp_types::Url,
    text: &str,
    line_index: &LineIndex,
    diagnostics: &[lsp_types::Diagnostic],
    action: AnalyzerAction,
) -> lsp_types::CodeAction {
    // Mark diagnostics emitted by the same rule as resolved by this action
    let diagnostics: Vec<_> = diagnostics
        .iter()
        .filter_map(|d| {
            let code = d.code.as_ref().and_then(|code| match code {
                lsp_types::NumberOrString::String(code) => Some(code.as_str()),
                lsp_types::NumberOrString::Number(_) => None,
            })?;

            if code == action.rule_name {
                Some(d.clone())
            } else {
                None
            }
        })
        .collect();

    let mut changes = HashMap::new();
    changes.insert(
        url.clone(),
        vec![lsp_types::TextEdit {
            range: lsp_types::Range::new(
                position(line_index, 0.into()),
                position(line_index, TextSize::of(text)),
            ),
            new_text: action.root.syntax().to_string(),
        }],
    );

    let edit = lsp_types::WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    };

    let is_safe_fix = action.category.contains(ActionCategories::SAFE_FIX);
    let is_suggestion = action.category.contains(ActionCategories::SUGGESTION);
    let is_refactor = action.category.contains(ActionCategories::REFACTOR);

    lsp_types::CodeAction {
        title: print_markup(&action.message),
        kind: if is_safe_fix || is_suggestion {
            Some(lsp_types::CodeActionKind::QUICKFIX)
        } else if is_refactor {
            Some(lsp_types::CodeActionKind::REFACTOR)
        } else {
            None
        },
        diagnostics: if !diagnostics.is_empty() {
            Some(diagnostics)
        } else {
            None
        },
        edit: Some(edit),
        command: None,
        is_preferred: if is_safe_fix { Some(true) } else { None },
        disabled: None,
        data: None,
    }
}

/// Convert an [rome_diagnostics::Diagnostic] to a [lsp_types::Diagnostic], using the span
/// of the diagnostic's primary label as the diagnostic range.
/// Requires a [LineIndex] to convert a byte offset range to the line/col range
/// expected by LSP.
pub(crate) fn diagnostic_to_lsp(
    diagnostic: AnalyzerDiagnostic,
    line_index: &LineIndex,
) -> lsp_types::Diagnostic {
    Diagnostic::new(
        range(line_index, diagnostic.range),
        Some(match diagnostic.severity {
            Severity::Help => lsp_types::DiagnosticSeverity::HINT,
            Severity::Note => lsp_types::DiagnosticSeverity::INFORMATION,
            Severity::Warning => lsp_types::DiagnosticSeverity::WARNING,
            Severity::Error | Severity::Bug => lsp_types::DiagnosticSeverity::ERROR,
        }),
        Some(lsp_types::NumberOrString::String(
            diagnostic.rule_name.into(),
        )),
        Some("rome".into()),
        print_markup(&diagnostic.message),
        None,
        None,
    )
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
pub(crate) fn into_lsp_error(msg: impl Display) -> LspError {
    let mut error = LspError::internal_error();
    error!("Error: {}", msg);
    error.data = Some(msg.to_string().into());
    error
}

/// Utility to spawn a task using [tokio::task::spawn_blocking] onto a thread intended
/// for blocking or compute-heavy tasks. The provided task must return a [Result] and
/// the result will be flattened to an [LspResult]
pub(crate) async fn spawn_blocking_task<F, R, E>(f: F) -> LspResult<R>
where
    F: FnOnce() -> Result<R, E> + Send + 'static,
    R: Send + 'static,
    E: Display + Send + 'static,
{
    match tokio::task::spawn_blocking(f).await {
        Ok(task_result) => task_result.map_err(into_lsp_error),
        Err(_) => Err(LspError::internal_error()),
    }
}
