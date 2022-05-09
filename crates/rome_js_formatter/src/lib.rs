//! Rome's official JavaScript formatter.

mod cst;
pub mod formatter;
mod js;
mod jsx;
pub mod prelude;
mod ts;
pub mod utils;

use crate::formatter::suppressed_node;
use crate::utils::has_formatter_suppressions;
pub(crate) use formatter::{format_leading_trivia, format_trailing_trivia, JsFormatter};
use rome_formatter::prelude::*;
use rome_formatter::{
    FormatOptions, FormatOwnedWithRule, FormatRefWithRule, Formatted, IndentStyle, Printed,
};
use rome_js_syntax::{JsLanguage, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::TextRange;
use rome_rowan::{AstNode, TextSize};
use rome_rowan::{SyntaxResult, TokenAtOffset};

use std::iter::FusedIterator;
use std::marker::PhantomData;

// Per Crate

/// Used to get an object that knows how to format this object.
pub trait AsFormat<'a> {
    type Format: Format;

    /// Returns an object that is able to format this object.
    fn format(&'a self) -> Self::Format;
}

/// Implement [AsFormat] for all types that have an associated [FormatRule].
impl<'a, T> AsFormat<'a> for &'a T
where
    T: AsFormat<'a>,
{
    type Format = T::Format;

    fn format(&'a self) -> Self::Format {
        AsFormat::format(&**self)
    }
}

/// Implement [AsFormat] for [SyntaxResult] where `T` implements [AsFormat].
///
/// Useful to format mandatory AST fields without having to unwrap the value first.
impl<'a, T> AsFormat<'a> for SyntaxResult<T>
where
    T: AsFormat<'a>,
{
    type Format = SyntaxResult<T::Format>;

    fn format(&'a self) -> Self::Format {
        match self {
            Ok(value) => Ok(value.format()),
            Err(err) => Err(*err),
        }
    }
}

/// Implement [AsFormat] for [Option] when `T` implements [AsFormat]
///
/// Allows to call format on optional AST fields without having to unwrap the field first.
impl<'a, T> AsFormat<'a> for Option<T>
where
    T: AsFormat<'a>,
{
    type Format = Option<T::Format>;

    fn format(&'a self) -> Self::Format {
        self.as_ref().map(|value| value.format())
    }
}

/// Used to convert this object into an object that can be formatted.
///
/// The difference to [AsFormat] is that this trait takes ownership of `self`.
pub trait IntoFormat {
    type Format: Format;

    fn into_format(self) -> Self::Format;
}

impl<T> IntoFormat for SyntaxResult<T>
where
    T: IntoFormat,
{
    type Format = SyntaxResult<T::Format>;

    fn into_format(self) -> Self::Format {
        self.map(IntoFormat::into_format)
    }
}

/// Implement [AsFormat] for [Option] when `T` implements [AsFormat]
///
/// Allows to call format on optional AST fields without having to unwrap the field first.
impl<T> IntoFormat for Option<T>
where
    T: IntoFormat,
{
    type Format = Option<T::Format>;

    fn into_format(self) -> Self::Format {
        self.map(IntoFormat::into_format)
    }
}

/// Formatting specific [Iterator] extensions
pub trait FormattedIterExt {
    /// Converts every item to an object that knows how to format it.
    fn formatted(self) -> FormattedIter<Self, Self::Item>
    where
        Self: Iterator + Sized,
        Self::Item: IntoFormat,
    {
        FormattedIter { inner: self }
    }
}

impl<I> FormattedIterExt for I where I: Iterator {}

pub struct FormattedIter<Iter, Item>
where
    Iter: Iterator<Item = Item>,
{
    inner: Iter,
}

impl<Iter, Item> Iterator for FormattedIter<Iter, Item>
where
    Iter: Iterator<Item = Item>,
    Item: IntoFormat,
{
    type Item = Item::Format;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.into_format())
    }
}

impl<Iter, Item> FusedIterator for FormattedIter<Iter, Item>
where
    Iter: FusedIterator<Item = Item>,
    Item: IntoFormat,
{
}

impl<Iter, Item> ExactSizeIterator for FormattedIter<Iter, Item>
where
    Iter: Iterator<Item = Item> + ExactSizeIterator,
    Item: IntoFormat,
{
}

pub struct FormatNodeRule<T>
where
    T: AstNode<Language = JsLanguage>,
{
    node: PhantomData<T>,
}

impl<N> FormatRule<N> for FormatNodeRule<N>
where
    N: AstNode<Language = JsLanguage>,
    FormatNodeRule<N>: FormatNodeFields<N>,
{
    fn format(node: &N, formatter: &Formatter) -> FormatResult<FormatElement> {
        let syntax = node.syntax();
        let element = if has_formatter_suppressions(syntax) {
            suppressed_node(syntax).format(formatter)?
        } else {
            Self::format_fields(node, formatter)?
        };

        Ok(element)
    }
}

pub trait FormatNodeFields<T>
where
    T: AstNode<Language = JsLanguage>,
{
    /// Formats the node's fields.
    fn format_fields(item: &T, formatter: &Formatter) -> FormatResult<FormatElement>;
}

/// Format implementation specific to JavaScript tokens.
pub struct FormatJsSyntaxToken;

impl FormatRule<JsSyntaxToken> for FormatJsSyntaxToken {
    fn format(token: &JsSyntaxToken, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.track_token(token);

        Ok(format_elements![
            format_leading_trivia(token, formatter::TriviaPrintMode::Full),
            Token::from(token),
            format_trailing_trivia(token),
        ])
    }
}

impl<'a> AsFormat<'a> for JsSyntaxToken {
    type Format = FormatRefWithRule<'a, JsSyntaxToken, FormatJsSyntaxToken>;

    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}

impl IntoFormat for JsSyntaxToken {
    type Format = FormatOwnedWithRule<JsSyntaxToken, FormatJsSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}

/// Formats a JavaScript (and its super languages) file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
pub fn format_node(options: FormatOptions, root: &JsSyntaxNode) -> FormatResult<Formatted> {
    tracing::trace_span!("format_node").in_scope(move || {
        let formatter = Formatter::new(options);
        let element = formatted![&formatter, root.format()]?;

        formatter.assert_formatted_all_tokens(root);

        Ok(Formatted::new(element, options))
    })
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
    root: &JsSyntaxNode,
    range: TextRange,
) -> FormatResult<Printed> {
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
            None => return Ok(Printed::new_empty()),
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
            None => return Ok(Printed::new_empty()),
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
    let formatted = format_sub_tree(options, common_root)?;

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
    let verbatim_ranges = Vec::from(formatted.verbatim_ranges());
    let code = &formatted.into_code()[output_range];
    Ok(Printed::new(
        code.into(),
        Some(input_range),
        sourcemap,
        verbatim_ranges,
    ))
}

/// Formats a single node within a file, supported by Rome.
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [FormatOptions], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Formatted] result
pub fn format_sub_tree(options: FormatOptions, root: &JsSyntaxNode) -> FormatResult<Printed> {
    // Determine the initial indentation level for the printer by inspecting the trivia pieces
    // of each token from the first token of the common root towards the start of the file
    let mut tokens = std::iter::successors(root.first_token(), |token| token.prev_token());

    // From the iterator of tokens, build an iterator of trivia pieces (once again the iterator is
    // reversed, starting from the last trailing trivia towards the first leading trivia).
    // The first token is handled specially as we only wan to consider its leading trivia pieces
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

    let formatted = format_node(options, root)?;
    let printed = formatted.print_with_indent(initial_indent);
    let sourcemap = Vec::from(printed.sourcemap());
    let verbatim_ranges = Vec::from(printed.verbatim_ranges());
    Ok(Printed::new(
        printed.into_code(),
        Some(root.text_range()),
        sourcemap,
        verbatim_ranges,
    ))
}

#[cfg(test)]
mod tests {

    use super::format_range;

    use rome_formatter::{FormatOptions, IndentStyle};
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
mod generated;

#[cfg(test)]
mod test {
    use crate::check_reformat::{check_reformat, CheckReformatParams};
    use crate::format_node;
    use rome_formatter::FormatOptions;
    use rome_js_parser::{parse, SourceType};

    #[test]
    #[ignore]
    // use this test check if your snippet prints as you wish, without using a snapshot
    fn quick_test() {
        let src = r#"xyz.a(b!).a(b!).a(b!)

"#;
        let syntax = SourceType::jsx();
        let tree = parse(src, 0, syntax.clone());
        let result = format_node(FormatOptions::default(), &tree.syntax())
            .unwrap()
            .print();
        check_reformat(CheckReformatParams {
            root: &tree.syntax(),
            text: result.as_code(),
            source_type: syntax,
            file_name: "quick_test",
            format_options: FormatOptions::default(),
        });
        assert_eq!(
            result.as_code(),
            r#"(a + (b * c)) > (65 + 5);
"#
        );
    }
}
