use crate::line_index::{LineCol, LineIndex};
use anyhow::{ensure, Context, Result};
use rome_console::fmt::Termcolor;
use rome_console::fmt::{self, Formatter};
use rome_console::MarkupBuf;
use rome_diagnostics::termcolor::NoColor;
use rome_diagnostics::{
    Applicability, {Diagnostic, DiagnosticTags, Location, PrintDescription, Severity, Visit},
};
use rome_rowan::{TextRange, TextSize};
use rome_service::workspace::CodeAction;
use rome_text_edit::{CompressedOp, DiffOp, TextEdit};
use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::io;
use tower_lsp::jsonrpc::Error as LspError;
use tower_lsp::lsp_types::{self as lsp};
use tracing::{debug, error};

pub(crate) fn position(line_index: &LineIndex, offset: TextSize) -> Result<lsp::Position> {
    let line_col = line_index.line_col(offset).with_context(|| {
        format!(
            "could not convert offset {offset:?} into a line-column index into string {:?}",
            line_index.text()
        )
    })?;

    Ok(lsp::Position::new(line_col.line, line_col.col))
}

pub(crate) fn range(line_index: &LineIndex, range: TextRange) -> Result<lsp::Range> {
    let start = position(line_index, range.start())?;
    let end = position(line_index, range.end())?;
    Ok(lsp::Range::new(start, end))
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

pub(crate) fn text_edit(line_index: &LineIndex, diff: TextEdit) -> Result<Vec<lsp::TextEdit>> {
    let mut result: Vec<lsp::TextEdit> = Vec::new();
    let mut offset = TextSize::from(0);

    for op in diff.iter() {
        match op {
            CompressedOp::DiffOp(DiffOp::Equal { range }) => {
                offset += range.len();
            }
            CompressedOp::DiffOp(DiffOp::Insert { range }) => {
                let start = position(line_index, offset)?;

                // Merge with a previous delete operation if possible
                let last_edit = result.last_mut().filter(|text_edit| {
                    text_edit.range.end == start && text_edit.new_text.is_empty()
                });

                if let Some(last_edit) = last_edit {
                    last_edit.new_text = diff.get_text(*range).to_string();
                } else {
                    result.push(lsp::TextEdit {
                        range: lsp::Range::new(start, start),
                        new_text: diff.get_text(*range).to_string(),
                    });
                }
            }
            CompressedOp::DiffOp(DiffOp::Delete { range }) => {
                let start = position(line_index, offset)?;
                offset += range.len();
                let end = position(line_index, offset)?;

                result.push(lsp::TextEdit {
                    range: lsp::Range::new(start, end),
                    new_text: String::new(),
                });
            }

            CompressedOp::EqualLines { line_count } => {
                let mut line_col = line_index
                    .line_col(offset)
                    .expect("diff length is overflowing the line count in the original file");

                line_col.line += line_count.get() + 1;
                line_col.col = 0;

                // SAFETY: This should only happen if `line_index` wasn't built
                // from the same string as the old revision of `diff`
                let new_offset = line_index
                    .offset(line_col)
                    .expect("diff length is overflowing the line count in the original file");

                offset = new_offset;
            }
        }
    }

    Ok(result)
}

pub(crate) fn code_fix_to_lsp(
    url: &lsp::Url,
    line_index: &LineIndex,
    diagnostics: &[lsp::Diagnostic],
    action: CodeAction,
) -> Result<lsp::CodeAction> {
    // Mark diagnostics emitted by the same rule as resolved by this action
    let diagnostics: Vec<_> = if action.category.matches("quickfix") {
        diagnostics
            .iter()
            .filter_map(|d| {
                let code = d.code.as_ref()?;
                let code = match code {
                    lsp::NumberOrString::String(code) => code.as_str(),
                    lsp::NumberOrString::Number(_) => return None,
                };

                    let code = code.strip_prefix("lint/")?;
                    let code = code.strip_prefix(group_name.as_ref())?;
                    let code = code.strip_prefix('/')?;

                    if code == rule_name {
                        Some(d.clone())
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let kind = action.category.to_str();
    let mut kind = kind.into_owned();

    if let Some((group, rule)) = action.rule_name {
        kind.push('.');
        kind.push_str(group.as_ref());
        kind.push('.');
        kind.push_str(rule.as_ref());
    }

    let suggestion = action.suggestion;

    let mut changes = HashMap::new();
    let edits = text_edit(line_index, suggestion.suggestion)?;

    changes.insert(url.clone(), edits);

    let edit = lsp::WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    };

    Ok(lsp::CodeAction {
        title: print_markup(&suggestion.msg),
        kind: Some(lsp::CodeActionKind::from(kind)),
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
    })
}

/// Convert an [rome_diagnostics::Diagnostic] to a [lsp::Diagnostic], using the span
/// of the diagnostic's primary label as the diagnostic range.
/// Requires a [LineIndex] to convert a byte offset range to the line/col range
/// expected by LSP.
pub(crate) fn diagnostic_to_lsp<D: Diagnostic>(
    diagnostic: D,
    url: &lsp::Url,
    line_index: &LineIndex,
) -> Result<lsp::Diagnostic> {
    let location = diagnostic
        .location()
        .context("diagnostic has no location")?;

    let span = location.span.context("diagnostic location has no span")?;
    let span = range(line_index, span).context("failed to convert diagnostic span to LSP range")?;

    let severity = match diagnostic.severity() {
        Severity::Fatal | Severity::Error => lsp::DiagnosticSeverity::ERROR,
        Severity::Warning => lsp::DiagnosticSeverity::WARNING,
        Severity::Information => lsp::DiagnosticSeverity::INFORMATION,
        Severity::Hint => lsp::DiagnosticSeverity::HINT,
    };

    let code = diagnostic
        .category()
        .map(|category| lsp::NumberOrString::String(category.name().to_string()));

    let message = PrintDescription(&diagnostic).to_string();
    ensure!(!message.is_empty(), "diagnostic description is empty");

    let mut related_information = None;
    let mut visitor = RelatedInformationVisitor {
        url,
        line_index,
        related_information: &mut related_information,
    };

    diagnostic.advices(&mut visitor).unwrap();

    let tags = diagnostic.tags();
    let tags = {
        let mut result = Vec::new();

        if tags.contains(DiagnosticTags::UNNECESSARY_CODE) {
            result.push(lsp::DiagnosticTag::UNNECESSARY);
        }

        if tags.contains(DiagnosticTags::DEPRECATED_CODE) {
            result.push(lsp::DiagnosticTag::DEPRECATED);
        }

        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    };

    Ok(lsp::Diagnostic::new(
        span,
        Some(severity),
        code,
        Some("rome".into()),
        message,
        related_information,
        tags,
    ))
}

struct RelatedInformationVisitor<'a> {
    url: &'a lsp::Url,
    line_index: &'a LineIndex,
    related_information: &'a mut Option<Vec<lsp::DiagnosticRelatedInformation>>,
}

impl Visit for RelatedInformationVisitor<'_> {
    fn record_frame(&mut self, location: Location<'_>) -> io::Result<()> {
        let span = match location.span {
            Some(span) => span,
            None => return Ok(()),
        };

        let range = match range(self.line_index, span) {
            Ok(range) => range,
            Err(_) => return Ok(()),
        };

        let related_information = self.related_information.get_or_insert_with(Vec::new);

        related_information.push(lsp::DiagnosticRelatedInformation {
            location: lsp::Location {
                uri: self.url.clone(),
                range,
            },
            message: String::new(),
        });

        Ok(())
    }
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

#[cfg(test)]
mod tests {
    use rome_text_edit::TextEdit;
    use tower_lsp::lsp_types as lsp;

    use crate::line_index::LineIndex;

    #[test]
    fn test_diff_1() {
        const OLD: &str = "line 1 old
line 2
line 3
line 4
line 5
line 6
line 7 old";

        const NEW: &str = "line 1 new
line 2
line 3
line 4
line 5
line 6
line 7 new";

        let line_index = LineIndex::new(OLD);
        let diff = TextEdit::from_unicode_words(OLD, NEW);

        let text_edit = super::text_edit(&line_index, diff).unwrap();

        assert_eq!(
            text_edit.as_slice(),
            &[
                lsp::TextEdit {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 0,
                            character: 7,
                        },
                        end: lsp::Position {
                            line: 0,
                            character: 10,
                        },
                    },
                    new_text: String::from("new"),
                },
                lsp::TextEdit {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 6,
                            character: 7
                        },
                        end: lsp::Position {
                            line: 6,
                            character: 10
                        }
                    },
                    new_text: String::from("new"),
                },
            ]
        );
    }

    #[test]
    fn test_diff_2() {
        const OLD: &str = "console.log(\"Variable: \" + variable);";
        const NEW: &str = "console.log(`Variable: ${variable}`);";

        let line_index = LineIndex::new(OLD);
        let diff = TextEdit::from_unicode_words(OLD, NEW);

        let text_edit = super::text_edit(&line_index, diff).unwrap();

        assert_eq!(
            text_edit.as_slice(),
            &[
                lsp::TextEdit {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 0,
                            character: 12,
                        },
                        end: lsp::Position {
                            line: 0,
                            character: 13,
                        },
                    },
                    new_text: String::from("`"),
                },
                lsp::TextEdit {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 0,
                            character: 23
                        },
                        end: lsp::Position {
                            line: 0,
                            character: 27
                        }
                    },
                    new_text: String::from("${"),
                },
                lsp::TextEdit {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 0,
                            character: 35
                        },
                        end: lsp::Position {
                            line: 0,
                            character: 35
                        }
                    },
                    new_text: String::from("}`"),
                },
            ]
        );
    }
}
