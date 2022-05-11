//! Rome's official JavaScript formatter.

mod cst;
mod format_extensions;
mod formatter;
mod js;
mod jsx;
#[macro_use]
pub mod macros;
pub mod prelude;
mod ts;
pub mod utils;

use crate::formatter::suppressed_node;
use crate::utils::has_formatter_suppressions;
pub use formatter::Formatter;
pub(crate) use formatter::{format_leading_trivia, format_trailing_trivia, JsFormatter};
pub use rome_formatter::{
    block_indent, comment, concat_elements, empty_element, empty_line, fill_elements,
    format_element, format_elements, group_elements, hard_group_elements, hard_line_break,
    if_group_breaks, if_group_fits_on_single_line, indent, join_elements, join_elements_hard_line,
    join_elements_soft_line, join_elements_with, line_suffix, soft_block_indent, soft_line_break,
    soft_line_break_or_space, soft_line_indent_or_space, space_token, token, FormatElement,
    FormatOptions, FormatResult, Formatted, IndentStyle, Printed, QuoteStyle, Token, Verbatim,
    LINE_TERMINATORS,
};
use rome_js_syntax::{JsLanguage, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::TextRange;
use rome_rowan::{AstNode, TextSize};
use rome_rowan::{SyntaxResult, TokenAtOffset};
use std::iter::FusedIterator;
use std::marker::PhantomData;

/// Formatting trait for types that can create a formatted representation. The `rome_formatter` equivalent
/// to [std::fmt::Display].
///
/// ## Example
/// Implementing `Format` for a custom struct
///
/// ```
/// use rome_formatter::{format_elements, FormatElement, FormatOptions, hard_line_break, Token, FormatResult};
/// use rome_js_formatter::{Format, format, formatted, Formatter};
/// use rome_rowan::TextSize;
///
/// struct Paragraph(String);
///
/// impl Format for Paragraph {fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
///         formatted![
///             formatter,
///             hard_line_break(),
///             FormatElement::from(Token::new_dynamic(self.0.clone(), TextSize::from(0))),
///             hard_line_break(),
///         ]
///     }
/// }
///
/// let paragraph = Paragraph(String::from("test"));
/// let printed = format(FormatOptions::default(), &paragraph).unwrap().print();
///
/// assert_eq!("test\n", printed.as_code())
/// ```
pub trait Format {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement>;
}

impl<T> Format for &T
where
    T: ?Sized + Format,
{
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Format::format(&**self, formatter)
    }
}

impl<T> Format for &mut T
where
    T: ?Sized + Format,
{
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Format::format(&**self, formatter)
    }
}

impl<T> Format for Option<T>
where
    T: Format,
{
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Some(value) => value.format(formatter),
            None => Ok(empty_element()),
        }
    }
}

impl<T> Format for SyntaxResult<T>
where
    T: Format,
{
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Ok(value) => value.format(formatter),
            Err(err) => Err(err.into()),
        }
    }
}

/// Implemented by traits that can be converted to a `FormatElement`.
///
/// This is similar to [Format] but with the difference that it consumes `self`, allowing it to also
/// be implemented on [FormatElement].format_elements.rs
pub trait IntoFormatElement {
    fn into_format_element(self, formatter: &Formatter) -> FormatResult<FormatElement>;
}

impl IntoFormatElement for FormatElement {
    fn into_format_element(self, _: &Formatter) -> FormatResult<FormatElement> {
        Ok(self)
    }
}

impl<T> IntoFormatElement for T
where
    T: Format,
{
    fn into_format_element(self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format(formatter)
    }
}

/// Rule that knows how to format an object of type [T].
///
/// Implementing [Format] on the object itself is preferred over implementing [FormatRule] but
/// this isn't possible inside of a dependent crate for external type.
///
/// For example, the `rome_js_formatter` crate isn't able to implement [Format] on `JsIfStatement`
/// because both the [Format] trait and `JsIfStatement` are external types (Rust's orphan rule).
///
/// That's why the `rome_js_formatter` crate must define a new-type that implements the formatting
/// of `JsIfStatement`.
pub trait FormatRule<T> {
    fn format(item: &T, formatter: &Formatter) -> FormatResult<FormatElement>;
}

// Per Crate

// The following types must pe per crate to work around Rust's orphan rule
// * `AsFormat` / `IntoFormat / WithFormatRule`: Traits must be crate local to implement them on the JS AST nodes
// * `FormattedIterExt`: Uses `IntoFormat`
// * `FormatNodeRule`: Defines language specific node formatting
// * `FormatWithRule`: To implement it for all traits implementing `AsFormat`.
// SyntaxNode + SyntaxToken -> Implement with FormatRule too.

// BOILERPLATE
// pub struct FormatJsMethodModifierList;
//
// impl FormatRule<JsMethodModifierList> for FormatJsMethodModifierList {
//     fn format(item: &JsMethodModifierList, formatter: &Formatter) -> FormatResult<FormatElement> {
//         todo!()
//     }
// }
//
// impl WithFormatRule for JsMethodModifierList {
//     type Rule = FormatJsMethodModifierList;
// }

/// Formats the referenced `item` with the specified rule.
pub struct FormatWithRule<'a, T, R>
where
    R: FormatRule<T>,
{
    item: &'a T,
    rule: PhantomData<R>,
}

impl<'a, T, R> FormatWithRule<'a, T, R>
where
    R: FormatRule<T>,
{
    pub fn new(item: &'a T) -> Self {
        Self {
            item,
            rule: PhantomData,
        }
    }
}

impl<T, R> Format for FormatWithRule<'_, T, R>
where
    R: FormatRule<T>,
{
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        R::format(self.item, formatter)
    }
}

/// Formats the `item` with the specified rule.
pub struct FormatedOwned<T>
where
    for<'a> T: AsFormat<'a>,
{
    item: T,
}

impl<T> FormatedOwned<T>
where
    for<'a> T: AsFormat<'a>,
{
    pub fn new(item: T) -> Self {
        Self { item }
    }
}

impl<T> Format for FormatedOwned<T>
where
    for<'a> T: AsFormat<'a>,
{
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.item.formatted().format(formatter)
    }
}

/// Trait implemented by object that have an associated [FormatRule] that knows how to format this object.
pub trait WithFormatRule: Sized {
    /// The rule that can format this object.
    type Rule: FormatRule<Self>;
}

/// Used to get an object that knows how to format this object.
pub trait AsFormat<'a> {
    type Format: Format;

    /// Returns an object that is able to format this object.
    fn formatted(&'a self) -> Self::Format;
}

/// Implement [AsFormat] for all types that have an associated [FormatRule].
impl<'a, T> AsFormat<'a> for T
where
    T: WithFormatRule + 'a,
{
    type Format = FormatWithRule<'a, T, T::Rule>;

    fn formatted(&'a self) -> Self::Format {
        FormatWithRule::new(self)
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

    fn formatted(&'a self) -> Self::Format {
        match self {
            Ok(value) => Ok(value.formatted()),
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

    fn formatted(&'a self) -> Self::Format {
        match self {
            None => None,
            Some(value) => Some(value.formatted()),
        }
    }
}

/// Used to convert this object into an object that can be formatted.
///
/// The difference to [AsFormat] is that this trait takes ownership of `self`.
pub trait IntoFormat {
    type Format: Format;

    fn into_format(self) -> Self::Format;
}

impl<T> IntoFormat for T
where
    for<'a> T: AsFormat<'a>,
{
    type Format = FormatedOwned<T>;

    fn into_format(self) -> Self::Format {
        FormatedOwned::new(self)
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

pub trait FormatNodeRule<T>
where
    T: AstNode<Language = JsLanguage>,
{
    fn format_fields(item: &T, formatter: &Formatter) -> FormatResult<FormatElement>;
}

impl<T, Item> FormatRule<Item> for T
where
    T: FormatNodeRule<Item>,
    Item: AstNode<Language = JsLanguage>,
{
    fn format(item: &Item, formatter: &Formatter) -> FormatResult<FormatElement> {
        let node = item.syntax();
        let element = if has_formatter_suppressions(node) {
            suppressed_node(node).format(formatter)?
        } else {
            Self::format_fields(item, formatter)?
        };

        Ok(element)
    }
}

/// Formatting trait for JS AST Nodes.
///
/// The code-gen generates a [Format] implementation for each `FormatNode` into the `format.rs` file.
pub trait FormatNode: AstNode<Language = JsLanguage> {
    /// Formats the node by calling into [FormatNode::format_fields] if the first token has no leading `rome-ignore` suppression comment.
    ///
    /// Formats the node "as is" if the node has a suppression comment.
    fn format_node(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let node = self.syntax();
        let element = if has_formatter_suppressions(node) {
            suppressed_node(node).format(formatter)?
        } else {
            self.format_fields(formatter)?
        };

        Ok(element)
    }

    /// Formats the node's fields.
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement>;
}

/// Format implementation specific to JavaScript tokens.
impl Format for JsSyntaxToken {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.track_token(self);

        Ok(format_elements![
            format_leading_trivia(self, formatter::TriviaPrintMode::Full),
            Token::from(self),
            format_trailing_trivia(self),
        ])
    }
}

/// Formats any value that implements [Format].
///
/// Please note that [format_node] is preferred to format a [JsSyntaxNode]
pub fn format(options: FormatOptions, root: &dyn Format) -> FormatResult<Formatted> {
    tracing::trace_span!("format").in_scope(move || {
        let formatter = Formatter::new(options);
        let element = root.format(&formatter)?;
        Ok(Formatted::new(element, options))
    })
}

/// Formats a JavaScript (and its super languages) file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
pub fn format_node(options: FormatOptions, root: &JsSyntaxNode) -> FormatResult<Formatted> {
    tracing::trace_span!("format_node").in_scope(move || {
        let formatter = Formatter::new(options);
        let element = root.format(&formatter)?;

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
mod generated;

#[cfg(test)]
mod test {
    use crate::check_reformat::{check_reformat, CheckReformatParams};
    use crate::format_node;
    use crate::FormatOptions;
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
