//! functions to convert diagnostics to LSP (language server protocol) Diagnostics.

use crate::file::Files;
use crate::*;

use lsp_types::{
    DiagnosticRelatedInformation, DiagnosticSeverity, DiagnosticTag as LspTag, Location,
    NumberOrString, Url,
};

pub fn convert_to_lsp_diagnostic(
    diagnostic: Diagnostic,
    files: &impl crate::file::Files,
    document_id: usize,
    uri: Url,
    source: Option<String>,
) -> Option<lsp_types::Diagnostic> {
    let mut related_information = vec![];

    let mut children = diagnostic
        .children
        .into_iter()
        .filter(|label| label.span.file == document_id)
        .collect::<Vec<_>>();

    children.sort_by(|a, b| a.span.range.clone().cmp(b.span.range.clone()));
    let mut primary_label = None;

    for child in children.into_iter().chain(diagnostic.primary.clone()) {
        let range = match byte_span_to_range(files, document_id, child.span.range.clone()) {
            Err(Error::ColumnOutOfBounds { max, .. }) => {
                let start = std::cmp::min(max, child.span.range.start);
                let end = std::cmp::min(max, child.span.range.end);
                byte_span_to_range(files, document_id, start..end)
            }
            range => range,
        }
        .ok()?;

        if Some(&child) == diagnostic.primary.as_ref() {
            primary_label = Some(range);
        }

        related_information.push(DiagnosticRelatedInformation {
            location: Location {
                uri: uri.clone(),
                range,
            },
            message: child.msg.clone(),
        });
    }

    let mut message = diagnostic.title;
    for footer in diagnostic.footers {
        let start = format!("\n{:#?}: ", footer.severity).to_ascii_lowercase();
        message.push_str(&start);
        message.push_str(&footer.msg);
    }

    let tags = diagnostic.tag.map(|tag| match tag {
        DiagnosticTag::Deprecated => vec![LspTag::Deprecated],
        DiagnosticTag::Unnecessary => vec![LspTag::Unnecessary],
        DiagnosticTag::Both => vec![LspTag::Deprecated, LspTag::Unnecessary],
    });

    Some(lsp_types::Diagnostic {
        range: primary_label?,
        severity: Some(severity_to_lsp_severity(diagnostic.severity)),
        code: diagnostic.code.map(NumberOrString::String),
        source,
        message,
        related_information: Some(related_information),
        tags,
    })
}

pub fn severity_to_lsp_severity(severity: Severity) -> DiagnosticSeverity {
    match severity {
        Severity::Error | Severity::Bug => DiagnosticSeverity::Error,
        Severity::Warning => DiagnosticSeverity::Warning,
        Severity::Help => DiagnosticSeverity::Hint,
        Severity::Note => DiagnosticSeverity::Information,
    }
}

// This code below is taken from codespan-lsp but adapted to use rome_diagnostics Files

use lsp_types::{Position as LspPosition, Range as LspRange};
use std::{error, fmt, ops::Range};

#[derive(Debug, PartialEq)]
pub enum Error {
    ColumnOutOfBounds { given: usize, max: usize },
    Location(LocationError),
    LineIndexOutOfBounds(LineIndexOutOfBoundsError),
    SpanOutOfBounds(SpanOutOfBoundsError),
    MissingFile,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ColumnOutOfBounds { given, max } => {
                write!(f, "Column out of bounds - given: {}, max: {}", given, max)
            }
            Error::Location(e) => e.fmt(f),
            Error::LineIndexOutOfBounds(e) => e.fmt(f),
            Error::SpanOutOfBounds(e) => e.fmt(f),
            Error::MissingFile => write!(f, "File does not exit"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LineIndexOutOfBoundsError {
    pub given: usize,
    pub max: usize,
}

impl error::Error for LineIndexOutOfBoundsError {}

impl fmt::Display for LineIndexOutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Line index out of bounds - given: {}, max: {}",
            self.given, self.max
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum LocationError {
    OutOfBounds { given: usize, span: Range<usize> },
    InvalidCharBoundary { given: usize },
}

impl error::Error for LocationError {}

impl fmt::Display for LocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationError::OutOfBounds { given, span } => write!(
                f,
                "Byte index out of bounds - given: {}, span: {}..{}",
                given, span.start, span.end
            ),
            LocationError::InvalidCharBoundary { given } => {
                write!(f, "Byte index within character boundary - given: {}", given)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SpanOutOfBoundsError {
    pub given: Range<usize>,
    pub span: Range<usize>,
}

impl error::Error for SpanOutOfBoundsError {}

impl fmt::Display for SpanOutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Span out of bounds - given: {}..{}, span: {}..{}",
            self.given.start, self.given.end, self.span.start, self.span.end
        )
    }
}

impl From<LocationError> for Error {
    fn from(e: LocationError) -> Error {
        Error::Location(e)
    }
}

impl From<LineIndexOutOfBoundsError> for Error {
    fn from(e: LineIndexOutOfBoundsError) -> Error {
        Error::LineIndexOutOfBounds(e)
    }
}

impl From<SpanOutOfBoundsError> for Error {
    fn from(e: SpanOutOfBoundsError) -> Error {
        Error::SpanOutOfBounds(e)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ColumnOutOfBounds { .. } | Error::MissingFile => None,
            Error::Location(error) => Some(error),
            Error::LineIndexOutOfBounds(error) => Some(error),
            Error::SpanOutOfBounds(error) => Some(error),
        }
    }
}

fn location_to_position(
    line_str: &str,
    line: usize,
    column: usize,
    byte_index: usize,
) -> Result<LspPosition, Error> {
    if column > line_str.len() {
        let max = line_str.len();
        let given = column;

        Err(Error::ColumnOutOfBounds { given, max })
    } else if !line_str.is_char_boundary(column) {
        let given = byte_index;

        Err(LocationError::InvalidCharBoundary { given }.into())
    } else {
        let line_utf16 = line_str[..column].encode_utf16();
        let character = line_utf16.count() as u64;
        let line = line as u64;

        Ok(LspPosition { line, character })
    }
}

pub fn byte_index_to_position<F>(
    files: &F,
    file_id: usize,
    byte_index: usize,
) -> Result<LspPosition, Error>
where
    F: Files,
{
    let source = files.source(file_id).ok_or(Error::MissingFile)?;

    let line_index =
        files
            .line_index(file_id, byte_index)
            .ok_or_else(|| LineIndexOutOfBoundsError {
                given: byte_index,
                max: source.lines().count(),
            })?;
    let line_span = files.line_range(file_id, line_index).unwrap();

    let line_str = source
        .get(line_span.clone())
        .ok_or_else(|| SpanOutOfBoundsError {
            given: line_span.clone(),
            span: 0..source.len(),
        })?;
    let column = byte_index - line_span.start;

    location_to_position(line_str, line_index, column, byte_index)
}

pub fn byte_span_to_range<F>(
    files: &F,
    file_id: usize,
    span: Range<usize>,
) -> Result<LspRange, Error>
where
    F: Files,
{
    Ok(LspRange {
        start: byte_index_to_position(files, file_id, span.start)?,
        end: byte_index_to_position(files, file_id, span.end)?,
    })
}

pub fn character_to_line_offset(line: &str, character: u64) -> Result<usize, Error> {
    let line_len = line.len();
    let mut character_offset = 0;

    let mut chars = line.chars();
    while let Some(ch) = chars.next() {
        if character_offset == character {
            let chars_off = chars.as_str().len();
            let ch_off = ch.len_utf8();

            return Ok(line_len - chars_off - ch_off);
        }

        character_offset += ch.len_utf16() as u64;
    }

    // Handle positions after the last character on the line
    if character_offset == character {
        Ok(line_len)
    } else {
        Err(Error::ColumnOutOfBounds {
            given: character_offset as usize,
            max: line.len(),
        })
    }
}

pub fn position_to_byte_index<F>(
    files: &F,
    file_id: usize,
    position: &LspPosition,
) -> Result<usize, Error>
where
    F: Files,
{
    let source = files.source(file_id).ok_or(Error::MissingFile)?;

    let line_span = files.line_range(file_id, position.line as usize).unwrap();
    let line_str = source.get(line_span.clone()).unwrap();

    let byte_offset = character_to_line_offset(line_str, position.character)?;

    Ok(line_span.start + byte_offset)
}

pub fn range_to_byte_span<F>(
    files: &F,
    file_id: usize,
    range: &LspRange,
) -> Result<Range<usize>, Error>
where
    F: Files,
{
    Ok(position_to_byte_index(files, file_id, &range.start)?
        ..position_to_byte_index(files, file_id, &range.end)?)
}
