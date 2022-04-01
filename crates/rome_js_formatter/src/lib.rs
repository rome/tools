//! Rome's official JavaScript formatter.

mod cst;
mod formatter;
mod formatter_traits;
mod js;
mod jsx;
pub mod prelude;
mod ts;
mod utils;
pub use formatter::Formatter;
pub use rome_formatter::intersperse::{Intersperse, IntersperseFn};
pub use rome_formatter::printer::{Printer, PrinterOptions};
pub use rome_formatter::{
    block_indent, comment, concat_elements, empty_element, empty_line, fill_elements,
    format_element, format_elements, group_elements, hard_group_elements, hard_line_break,
    if_group_breaks, if_group_fits_on_single_line, indent, join_elements, join_elements_hard_line,
    join_elements_soft_line, join_elements_with, line_suffix, soft_block_indent, soft_line_break,
    soft_line_break_or_space, soft_line_indent_or_space, space_token, token, FormatElement,
    FormatOptions, Formatted, IndentStyle, Token, Verbatim, LINE_TERMINATORS,
};
use rome_js_syntax::{SyntaxError, SyntaxNode};
use rome_rowan::TextRange;
use rome_rowan::TextSize;
use rome_rowan::TokenAtOffset;
use thiserror::Error;

/// This trait should be implemented on each node/value that should have a formatted representation
pub trait ToFormatElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement>;
}

/// Public return type of the formatter
pub type FormatResult<F> = Result<F, FormatError>;

#[derive(Debug, PartialEq, Error)]
/// Series of errors encountered during formatting
pub enum FormatError {
    /// Node is missing and it should be required for a correct formatting
    #[error("missing required child")]
    MissingRequiredChild,

    /// In case our formatter doesn't know how to format a certain language
    #[error("language is not supported")]
    UnsupportedLanguage,

    /// When the ability to format the current file has been turned off on purpose
    #[error("formatting capability is disabled")]
    CapabilityDisabled,
}

impl From<SyntaxError> for FormatError {
    fn from(syntax_error: SyntaxError) -> Self {
        match syntax_error {
            SyntaxError::MissingRequiredChild(_node) => FormatError::MissingRequiredChild,
        }
    }
}

impl From<&SyntaxError> for FormatError {
    fn from(syntax_error: &SyntaxError) -> Self {
        match syntax_error {
            SyntaxError::MissingRequiredChild(_node) => FormatError::MissingRequiredChild,
        }
    }
}

/// Formats a JavaScript (and its super languages) file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
#[tracing::instrument(level = "trace", skip_all)]
pub fn format(options: FormatOptions, syntax: &SyntaxNode) -> FormatResult<Formatted> {
    let element = Formatter::new(options).format_root(syntax)?;
    Ok(Printer::new(options).print(&element))
}

/// Outputs formatter IR for a JavaScript (and its super languages) file
///
/// It returns a [FormatElement] result. Mostly for debugging purposes.
pub fn to_format_element(
    options: FormatOptions,
    syntax: &SyntaxNode,
) -> FormatResult<FormatElement> {
    Formatter::new(options).format_root(syntax)
}

/// Formats a range within a file, supported by Rome
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [FormatOptions], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Formatted] result with a range corresponding to the
/// range of the input that was effectively overwritten by the formatter
pub fn format_range(
    options: FormatOptions,
    root: &SyntaxNode,
    range: TextRange,
) -> FormatResult<Formatted> {
    // Find the tokens corresponding to the start and end of the range
    let start_token = root.token_at_offset(range.start());
    let end_token = root.token_at_offset(range.end());

    // If these tokens were not found this means either:
    // 1. The input [SyntaxNode] was empty
    // 2. The input node was not the root [SyntaxNode] of the file
    // In the first case we can return an empty result immediately,
    // otherwise default to the first and last tokens in the root node
    let start_token = match start_token {
        // If the start of the range lies between two tokens,
        // start at the rightmost one
        TokenAtOffset::Between(_, token) => token,
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::None => match root.first_token() {
            Some(token) => token,
            // root node is empty
            None => return Ok(Formatted::new_empty()),
        },
    };
    let end_token = match end_token {
        // If the end of the range lies between two tokens,
        // end at the leftmost one
        TokenAtOffset::Between(token, _) => token,
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::None => match root.last_token() {
            Some(token) => token,
            // root node is empty
            None => return Ok(Formatted::new_empty()),
        },
    };

    // Find the lowest common ancestor node for the start and end token
    // by building the path to the root node from both tokens and
    // iterating along the two paths at once to find the first divergence
    #[allow(clippy::needless_collect)]
    let start_to_root: Vec<_> = start_token.ancestors().collect();
    #[allow(clippy::needless_collect)]
    let end_to_root: Vec<_> = end_token.ancestors().collect();

    let common_root = start_to_root
        .into_iter()
        .rev()
        .zip(end_to_root.into_iter().rev())
        .map_while(|(lhs, rhs)| if lhs == rhs { Some(lhs) } else { None })
        .last();

    // Logically this should always return at least the root node,
    // fallback to said node just in case
    let common_root = common_root.as_ref().unwrap_or(root);

    // Perform the actual formatting of the root node with
    // an appropriate indentation level
    let formatted = format_node(options, common_root)?;

    // This finds the closest marker to the beginning of the source
    // starting before or at said starting point, and the closest
    // marker to the end of the source range starting after or at
    // said ending point respectively
    let mut range_start = None;
    let mut range_end = None;

    let sourcemap = Vec::from(formatted.sourcemap());
    for marker in &sourcemap {
        if let Some(start_dist) = marker.source.checked_sub(range.start()) {
            range_start = match range_start {
                Some((prev_marker, prev_dist)) => {
                    if start_dist < prev_dist {
                        Some((marker, start_dist))
                    } else {
                        Some((prev_marker, prev_dist))
                    }
                }
                None => Some((marker, start_dist)),
            }
        }

        if let Some(end_dist) = range.end().checked_sub(marker.source) {
            range_end = match range_end {
                Some((prev_marker, prev_dist)) => {
                    if end_dist < prev_dist {
                        Some((marker, end_dist))
                    } else {
                        Some((prev_marker, prev_dist))
                    }
                }
                None => Some((marker, end_dist)),
            }
        }
    }

    // If no start or end were found, this means that the edge of the formatting
    // range was near the edge of the input, and no marker were emitted before
    // the start (or after the end) of the formatting range: in this case
    // the start/end marker default to the start/end of the input
    let (start_source, start_dest) = match range_start {
        Some((start_marker, _)) => (start_marker.source, start_marker.dest),
        None => (common_root.text_range().start(), TextSize::from(0)),
    };
    let (end_source, end_dest) = match range_end {
        Some((end_marker, _)) => (end_marker.source, end_marker.dest),
        None => (
            common_root.text_range().end(),
            TextSize::try_from(formatted.as_code().len()).expect("code length out of bounds"),
        ),
    };

    let input_range = TextRange::new(start_source, end_source);
    let output_range = TextRange::new(start_dest, end_dest);
    let sourcemap = Vec::from(formatted.sourcemap());
    let verbatim = Vec::from(formatted.verbatim());
    let code = &formatted.into_code()[output_range];
    Ok(Formatted::new(
        code.into(),
        Some(input_range),
        sourcemap,
        verbatim,
    ))
}

/// Formats a single node within a file, supported by Rome
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [FormatOptions], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Formatted] result
pub fn format_node(options: FormatOptions, root: &SyntaxNode) -> FormatResult<Formatted> {
    // Determine the initial indentation level for the printer by inspecting the trivias
    // of each token from the first token of the common root towards the start of the file
    let mut tokens = std::iter::successors(root.first_token(), |token| token.prev_token());

    // From the iterator of tokens, build an iterator of trivia pieces (once again the iterator is
    // reversed, starting from the last trailing trivia towards the first leading trivia).
    // The first token is handled specially as we only wan to consider its leading trivias
    let first_token = tokens.next();
    let first_token_trivias = first_token
        .into_iter()
        .flat_map(|token| token.leading_trivia().pieces().rev());

    let next_tokens_trivias = tokens.flat_map(|token| {
        token
            .trailing_trivia()
            .pieces()
            .rev()
            .chain(token.leading_trivia().pieces().rev())
    });

    let trivias = first_token_trivias
        .chain(next_tokens_trivias)
        .filter(|piece| {
            // We're only interested in newline and whitespace trivias, skip over comments
            let is_newline = piece.is_newline();
            let is_whitespace = piece.is_whitespace();
            is_newline || is_whitespace
        });

    // Finally run the iterator until a newline trivia is found, and get the last whitespace trivia before it
    let last_whitespace = trivias.map_while(|piece| piece.as_whitespace()).last();
    let initial_indent = match last_whitespace {
        Some(trivia) => {
            // This logic is based on the formatting options passed in
            // the be user (or the editor) as we do not have any kind
            // of indentation type detection yet. Unfortunately this
            // may not actually match the current content of the file
            let length = trivia.text().len() as u16;
            match options.indent_style {
                IndentStyle::Tab => length,
                IndentStyle::Space(width) => length / u16::from(width),
            }
        }
        // No whitespace was found between the start of the range
        // and the start of the file
        None => 0,
    };

    let element = Formatter::new(options).format_root(root)?;
    let formatted = Printer::new(options).print_with_indent(&element, initial_indent);
    let sourcemap = Vec::from(formatted.sourcemap());
    let verbatim = Vec::from(formatted.verbatim());
    Ok(Formatted::new(
        formatted.into_code(),
        Some(root.text_range()),
        sourcemap,
        verbatim,
    ))
}

#[cfg(test)]
mod tests {

    use super::{format_range, FormatOptions};
    use crate::IndentStyle;
    use rome_js_parser::parse_script;
    use rome_rowan::{TextRange, TextSize};

    #[test]
    fn test_range_formatting() {
        let input = "
while(
    true
) {
    function func() {
    func(     /* comment */
    );

    let array =
        [ 1
    , 2];

    }

    function func2()
    {

    const no_format    =    () => {};

    }
}
";

        // Start the formatting range two characters before the "let" keywords,
        // in the middle of the indentation whitespace for the line
        let range_start = TextSize::try_from(input.find("let").unwrap() - 2).unwrap();
        let range_end = TextSize::try_from(input.find("const").unwrap()).unwrap();

        let tree = parse_script(input, 0);
        let result = format_range(
            FormatOptions {
                indent_style: IndentStyle::Space(4),
                ..FormatOptions::default()
            },
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );

        let result = result.expect("range formatting failed");
        assert_eq!(
            result.range(),
            Some(TextRange::new(range_start + TextSize::from(2), range_end))
        );
        assert_eq!(
            result.as_code(),
            "let array = [1, 2];\n    }\n\n    function func2() {\n        "
        );
    }

    #[test]
    fn test_range_formatting_indentation() {
        let input = "
function() {
         const veryLongIdentifierToCauseALineBreak = { veryLongKeyToCauseALineBreak: 'veryLongValueToCauseALineBreak' }
}
";

        let range_start = TextSize::try_from(input.find("const").unwrap()).unwrap();
        let range_end = TextSize::try_from(input.find('}').unwrap()).unwrap();

        let tree = parse_script(input, 0);
        let result = format_range(
            FormatOptions {
                indent_style: IndentStyle::Space(4),
                ..FormatOptions::default()
            },
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );

        let result = result.expect("range formatting failed");
        assert_eq!(result.range(), Some(TextRange::new(range_start, range_end)));
        // As a result of the indentation normalization, the number of spaces within
        // the object expression is currently rounded down from an odd indentation level
        assert_eq!(
            result.as_code(),
            "const veryLongIdentifierToCauseALineBreak = {\n            veryLongKeyToCauseALineBreak: \"veryLongValueToCauseALineBreak\",\n        "
        );
    }
}

#[cfg(test)]
mod check_reformat;

#[cfg(test)]
mod test {
    use crate::check_reformat::{check_reformat, CheckReformatParams};
    use crate::format;
    use crate::FormatOptions;
    use rome_js_parser::{parse, SourceType};

    #[test]
    #[ignore]
    // use this test check if your snippet prints as you wish, without using a snapshot
    fn quick_test() {
        let src = r#"
 const functionName1 = <T,>(arg) => false;
"#;
        let syntax = SourceType::tsx();
        let tree = parse(src, 0, syntax.clone());
        let result = format(FormatOptions::default(), &tree.syntax()).unwrap();
        check_reformat(CheckReformatParams {
            root: &tree.syntax(),
            text: result.as_code(),
            source_type: syntax,
            file_name: "quick_test",
            format_options: FormatOptions::default(),
        });
        assert_eq!(
            result.as_code(),
            r#"let g = [[], [0, 1], [0, 1]];
"#
        );
    }
}
