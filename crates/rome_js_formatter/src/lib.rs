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
use rome_formatter::{FormatOwnedWithRule, FormatRefWithRule, Formatted, Printed};
use rome_js_syntax::{JsLanguage, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::AstNode;
use rome_rowan::SyntaxResult;
use rome_rowan::TextRange;

use crate::cst::FormatJsSyntaxNode;
use crate::options::JsFormatOptions;
use std::iter::FusedIterator;
use std::marker::PhantomData;

// Per Crate

/// Used to get an object that knows how to format this object.
pub trait AsFormat<'a> {
    type Format: Format<Options = JsFormatOptions>;

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
    type Options = JsFormatOptions;

    fn format(node: &N, formatter: &Formatter<JsFormatOptions>) -> FormatResult<FormatElement> {
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
    fn format_fields(
        item: &T,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement>;
}

/// Format implementation specific to JavaScript tokens.
pub struct FormatJsSyntaxToken;

impl FormatRule<JsSyntaxToken> for FormatJsSyntaxToken {
    type Options = JsFormatOptions;

    fn format(
        token: &JsSyntaxToken,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
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
    options: JsFormatOptions,
    root: &JsSyntaxNode,
    range: TextRange,
) -> FormatResult<Printed> {
    rome_formatter::format_range::<_, _, FormatJsSyntaxNode>(options, root, range)
}

/// Formats a JavaScript (and its super languages) file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
pub fn format_node(options: JsFormatOptions, root: &JsSyntaxNode) -> FormatResult<Formatted> {
    rome_formatter::format_node(options, &root.format())
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
pub fn format_sub_tree(options: JsFormatOptions, root: &JsSyntaxNode) -> FormatResult<Printed> {
    rome_formatter::format_sub_tree(options, &root.format())
}

#[cfg(test)]
mod tests {

    use super::format_range;

    use crate::options::JsFormatOptions;
    use rome_formatter::IndentStyle;
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
            JsFormatOptions {
                indent_style: IndentStyle::Space(4),
                ..JsFormatOptions::default()
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
            JsFormatOptions {
                indent_style: IndentStyle::Space(4),
                ..JsFormatOptions::default()
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
#[rustfmt::skip]
mod generated;
pub mod options;

#[cfg(test)]
mod test {
    use crate::check_reformat::{check_reformat, CheckReformatParams};
    use crate::{format_node, JsFormatOptions};
    use rome_js_parser::{parse, SourceType};

    #[test]
    #[ignore]
    // use this test check if your snippet prints as you wish, without using a snapshot
    fn quick_test() {
        let src = r#"
        const AspectRatioBox = ({
  aspectRatio,
  children,
  ...props
}) => (
  <div
    className={`height: 0;
  overflow: hidden;
  padding-top: ${props => 100 / props.aspectRatio}%;
  background: white;
  position: relative;`}
  >
    <div>{children}</div>
  </div>
);
        "#;
        let syntax = SourceType::jsx();
        let tree = parse(src, 0, syntax.clone());
        let result = format_node(JsFormatOptions::default(), &tree.syntax())
            .unwrap()
            .print();
        check_reformat(CheckReformatParams {
            root: &tree.syntax(),
            text: result.as_code(),
            source_type: syntax,
            file_name: "quick_test",
            format_options: JsFormatOptions::default(),
        });

        println!("{}", result.as_code());
    }
}
