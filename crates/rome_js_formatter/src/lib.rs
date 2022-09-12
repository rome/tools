//! Rome's official JavaScript formatter.
//!
//! ## Implement the formatter
//!`
//! Our formatter is node based. Meaning that each AST node knows how to format itself. In order to implement
//! the formatting, a node has to implement the trait `FormatNode`.
//!
//! `rome` has an automatic code generation that creates automatically the files out of the grammar.
//! By default, all implementations will format verbatim,
//! meaning that the formatter will print tokens and trivia as they are (`format_verbatim`).
//!
//! Our formatter has its own [internal IR](https://en.wikipedia.org/wiki/Intermediate_representation), it creates its own abstraction from an AST.
//!
//! The developer won't be creating directly this IR, but they will use a series of utilities that will help
//! to create this IR. The whole IR is represented by the `enum` `FormatElement`. Please refer to [its internal
//! documentation](#build-the-documentation) to understand the meaning of each variant.
//!
//!
//!
//! ### Rules to follow when implementing a formatter
//!
//! 1. Use the `*Fields` struct to extract all the tokens/nodes
//!     ```rust,ignore
//!     #[derive(Debug, Clone, Default)]
//!     pub struct FormatJsExportDefaultExpressionClause;
//!
//!     impl FormatNodeRule<JsExportDefaultExpressionClause> for FormatJsExportDefaultExpressionClauses {
//!         fn fmt_fields(&self, node: &JsExportDefaultExpressionClause, f: &mut JsFormatter) -> FormatResult<()> {
//!             let JsExportDefaultExpressionClauseFields {
//!                 default_token,
//!                 expression,
//!                 semicolon_token,
//!             }  = node.as_fields();
//!        }
//!     }
//!     ```
//! 2. When using `.as_fields()` with the destructuring, don't use the `..` feature. Prefer extracting all fields and ignore them
//!    using the `_`
//!    ```rust,ignore
//!    #[derive(Debug, Clone, Default)]
//!    pub struct FormatJsExportDefaultExpressionClause;
//!
//!    impl FormatNodeRule<JsExportDefaultExpressionClause> for FormatJsExportDefaultExpressionClauses {
//!        fn fmt_fields(&self, node: &JsExportDefaultExpressionClause, f: &mut JsFormatter) -> FormatResult<()> {
//!             let JsExportDefaultExpressionClauseFields {
//!                 default_token,
//!                 expression: _,
//!                 semicolon_token
//!             } = node.as_fields();
//!         }
//!    }
//!    ```
//!    The reason why we want to promote this pattern is because we want to make explicit when a token/node is excluded;
//! 3. Use the APIs provided by `builders.rs`, `formatter` and `format_extensions.rs`.
//!    1. `builders.rs` exposes a series of utilities to craft the formatter IR; please refer to their internal
//!    documentation to understand what the utilities are for;
//!    2. `formatter` exposes a set of functions to help to format some recurring patterns; please refer to their internal
//!    documentation to understand how to use them and when;
//!    3. `format_extensions.rs`: with these traits, we give the ability to nodes and tokens to implements certain methods
//!    that are exposed based on its type. If you have a good IDE support, this feature will help you. For example:
//!    ```rust,ignore
//!    #[derive(Debug, Clone, Default)]
//!    pub struct FormatJsExportDefaultExpressionClause;
//!
//!    impl FormatNodeRule<JsExportDefaultExpressionClause> for FormatJsExportDefaultExpressionClauses{
//!         fn fmt_fields(&self, node: &JsExportDefaultExpressionClause, f: &mut JsFormatter) -> FormatResult<()> {
//!             let JsExportDefaultExpressionClauseFields {
//!                 default_token,
//!                 expression, // it's a mandatory node
//!                 semicolon_token, // this is not a mandatory node
//!             } = node.as_fields();
//!             let element = expression.format();
//!
//!             if let Some(expression) = &expression? {
//!                 write!(f, [expression.format(), space_token()])?;
//!             }
//!
//!             if let Some(semicolon) = &semicolon_token {
//!                 write!(f, [semicolon.format()])?;
//!             } else {
//!                 write!(f, [space_token()])?;
//!             }
//!         }
//!    }
//!    ```
//!
//! 4. Use our [playground](https://play.rome.tools) to inspect the code that you want to format. You can inspect
//! the AST given by a certain snippet. This will help you to understand which nodes need to be implemented/modified
//! in order to implement formatting. Alternatively, you can locally run the playground by following
//! the [playground instructions](https://github.com/rome/tools/blob/main/website/playground/README.md).
//! 5. Use the [`quick_test()`](https://github.com/rome/tools/blob/main/crates/rome_js_formatter/src/lib.rs#L597-L616)
//! function to test you snippet straight from your IDE, without running the whole test suite. The test
//! is ignored on purpose, so you won't need to worry about the CI breaking.
//!
//! ## Write tests for the formatter
//!
//! We use [insta.rs](https://insta.rs/docs) for our snapshot tests, please make sure you read its documentation to learn the basics of snapshot testing.
//! You should install the companion [`cargo-insta`](https://insta.rs/docs/cli/) command to assist with snapshot reviewing.
//!
//! Directories are divided by language, so when creating a new test file, make sure to have the correct file
//! under the correct folder:
//! - `JavaScript` => `js/` directory
//! - `TypeScript` => `ts/` directory
//! - `JSX` => `jsx/` directory
//! - `TSX` => `ts/` directory
//!
//! To create a new snapshot test for JavaScript, create a new file to `crates/rome_js_formatter/tests/specs/js/`, e.g. `arrow_with_spaces.js`
//!
//! ```javascript
//! const foo     = ()    => {
//!     return bar
//! }
//! ```
//!
//! Files processed as modules must go inside the `module/` directory, files processed as script must go inside the
//! `script/` directory.
//!
//! Run the following command to generate the new snapshot (the snapshot tests are generated by a procedure macro so we need to recompile the tests):
//!
//! ```bash
//! touch crates/rome_js_formatter/tests/spec_tests.rs && cargo test -p rome_js_formatter formatter
//! ```
//!
//! For better test driven development flow, start the formatter tests with [`cargo-watch`](https://crates.io/crates/cargo-watch):
//!
//! ```bash
//! cargo watch -i '*.new' -x 'test -p rome_js_formatter formatter'
//! ```
//!
//! After test execution, you will get a new `arrow.js.snap.new` file.
//!
//! To actually update the snapshot, run `cargo insta review` to interactively review and accept the pending snapshot. `arrow.js.snap.new` will be replaced with `arrow.js.snap`
//!
//! Sometimes, you need to verify the formatting for different cases/options. In order to do that, create a folder with
//! the cases you need to verify. If we needed to follow the previous example:
//!
//! 1. create a folder called `arrow_with_spaces/` and move the JS file there;
//! 2. then create a file called `options.json`
//! 3. The content would be something like:
//!     ```json
//!     {
//!         "cases": [
//!             {
//!                 "line_width": 120,
//!                 "indent_style": {"Space": 4}
//!             }
//!         ]
//!     }
//!     ````
//! 4. the `cases` keyword is mandatory;
//! 5. then each object of the array will contain the matrix of options you'd want to test.
//!    In this case the test suite will run a **second test case** with `line_width` to 120 and `ident_style` with  4 spaces
//! 6. when the test suite is run, you will have two outputs in your snapshot: the default one and the custom one
//!
//! ## Identify issues
//!
//! There are four cases when a test is not correct:
//! - you try to print/format the same token multiple times; the formatter will check at runtime when a test is run;
//! - some tokens haven't been printed; usually you will have this information inside the snapshot, under a section
//! called `"Unimplemented tokens/nodes"`; a test, in order to be valid, can't have that section;
//!
//!    If removing a token is the actual behaviour (removing some parenthesis or a semicolon), then the correct way
//!    to do it by using the formatter API `formatter.format_replaced(token, empty_element())`;
//! - the emitted code is not a valid program anymore, the test suite will parse again the emitted code and it will
//! fail if there are syntax errors;
//! - the emitted code, when formatted again, differs from the original; this usually happens when removing/adding new
//! elements, and the grouping is not correctly set;
//!
//!
//! ## Write tests for a parser
//!
//! If you want to create a new test for an existing parser, you will have to inline
//! the code that you want to test in a comment that is created in a specific way.
//!
//! Let's say that you created a new parsing feature and you need new tests from scratch,
//! just go to the source code where you parse this new feature if JavaScript, and add the following comment:
//!
//! ```rust,ignore
//! // test feature_name
//! // let a = { new_feature : "" }
//! // let b = { new_feature : "" }
//! fn parse_new_feature(p: &mut Parser) -> ParsedSyntax {}
//! ```
//!
//! The first line, `// test feature_name` the important one. This will tell to the
//! testing infrastructure to create a **positive test** (without parsing errors), called
//! `feature_name.js` inside the `test_data/inline/ok` folder.
//!
//! The content of this file will be:
//!
//! ```js
//! let a = { new_feature : "" }
//! let b = { new_feature : "" }
//! ```
//!
//! Basically, everything after the key comment will be the content of the new file.
//!
//! Now you need to run `cargo codegen test` and the task will actually generate this file for you.
//!
//! In case you want to create a **negative test** (*with* parsing errors), you will
//! create a new comment like this:
//!
//! ```diff
//! // test feature_name
//! // let a = { new_feature : "" }
//! // let b = { new_feature : "" }
//!
//! + // test_err feature_name
//! + // let a = {  : "" }
//! + // let b = { new_feature :  }
//! fn parse_new_feature(p: &mut Parser) -> ParsedSyntax {}
//! ```
//!
//! Mind the different comment **`test_err`**, which marks the error for the test suite
//! as a test that has to fail.
//!
//! Run the command `cargo codegen test` and you will see a new file called
//! `feature_name.js` inside the `test_data/inline/err` folder.
//!
//! The content of this file will be:
//!
//! ```js
//! let a = {  : "" }
//! let b = { new_feature :  }
//! ```
//!
//! Now run the command:
//! Unix/macOS
//!
//! ```bash
//! env UPDATE_EXPECT=1 cargo test
//! ```
//!
//! Windows
//!
//! ```powershell
//! set UPDATE_EXPECT=1 & cargo test
//! ```
//! The command will tell the test suite to generate and update the `.rast` files.
//!
//! If tests that are inside the `ok/` folder fail or if tests that are inside the `err/`
//! folder don't emit, the whole test suite will fail.

mod cst;
mod js;
mod jsx;
pub mod prelude;
mod ts;
pub mod utils;

#[cfg(test)]
mod check_reformat;
#[rustfmt::skip]
mod generated;
pub(crate) mod builders;
pub mod comments;
pub mod context;
mod parentheses;
pub(crate) mod separated;
mod syntax_rewriter;

use rome_formatter::prelude::*;
use rome_formatter::{
    write, Comments, CstFormatContext, Format, FormatLanguage, TransformSourceMap,
};
use rome_formatter::{Buffer, FormatOwnedWithRule, FormatRefWithRule, Formatted, Printed};
use rome_js_syntax::{
    JsAnyDeclaration, JsAnyStatement, JsLanguage, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
};
use rome_rowan::SyntaxResult;
use rome_rowan::TextRange;
use rome_rowan::{AstNode, SyntaxNode};

use crate::builders::{format_suppressed_node, format_unknown_node};
use crate::comments::JsCommentStyle;
use crate::context::{JsFormatContext, JsFormatOptions};
use crate::cst::FormatJsSyntaxNode;
use crate::syntax_rewriter::transform;
use rome_formatter::token::format_skipped_token_trivia;
use std::iter::FusedIterator;
use std::marker::PhantomData;

pub(crate) type JsFormatter<'buf> = Formatter<'buf, JsFormatContext>;

// Per Crate

/// Used to get an object that knows how to format this object.
pub trait AsFormat<'a> {
    type Format: Format<JsFormatContext>;

    /// Returns an object that is able to format this object.
    fn format(&'a self) -> Self::Format;
}

/// Implement [AsFormat] for references to types that implement [AsFormat].
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
pub trait IntoFormat<Context> {
    type Format: Format<Context>;

    fn into_format(self) -> Self::Format;
}

impl<T, Context> IntoFormat<Context> for SyntaxResult<T>
where
    T: IntoFormat<Context>,
{
    type Format = SyntaxResult<T::Format>;

    fn into_format(self) -> Self::Format {
        self.map(IntoFormat::into_format)
    }
}

/// Implement [IntoFormat] for [Option] when `T` implements [IntoFormat]
///
/// Allows to call format on optional AST fields without having to unwrap the field first.
impl<T, Context> IntoFormat<Context> for Option<T>
where
    T: IntoFormat<Context>,
{
    type Format = Option<T::Format>;

    fn into_format(self) -> Self::Format {
        self.map(IntoFormat::into_format)
    }
}

/// Formatting specific [Iterator] extensions
pub trait FormattedIterExt {
    /// Converts every item to an object that knows how to format it.
    fn formatted<Context>(self) -> FormattedIter<Self, Self::Item, Context>
    where
        Self: Iterator + Sized,
        Self::Item: IntoFormat<Context>,
    {
        FormattedIter {
            inner: self,
            options: PhantomData,
        }
    }
}

impl<I> FormattedIterExt for I where I: Iterator {}

pub struct FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item>,
{
    inner: Iter,
    options: PhantomData<Context>,
}

impl<Iter, Item, Context> Iterator for FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item>,
    Item: IntoFormat<Context>,
{
    type Item = Item::Format;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.into_format())
    }
}

impl<Iter, Item, Context> FusedIterator for FormattedIter<Iter, Item, Context>
where
    Iter: FusedIterator<Item = Item>,
    Item: IntoFormat<Context>,
{
}

impl<Iter, Item, Context> ExactSizeIterator for FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item> + ExactSizeIterator,
    Item: IntoFormat<Context>,
{
}

pub trait FormatNodeRule<N>
where
    N: AstNode<Language = JsLanguage>,
{
    fn fmt(&self, node: &N, f: &mut JsFormatter) -> FormatResult<()> {
        if self.is_suppressed(node, f) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        self.fmt_leading_comments(node, f)?;

        if self.needs_parentheses(node) {
            write!(
                f,
                [
                    text("("),
                    format_once(|f| self.fmt_fields(node, f)),
                    text(")"),
                ]
            )?;
        } else {
            self.fmt_fields(node, f)?;
        }

        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    /// Formats the node's fields.
    fn fmt_fields(&self, item: &N, f: &mut JsFormatter) -> FormatResult<()>;

    /// Returns whether the node requires parens.
    fn needs_parentheses(&self, item: &N) -> bool {
        let _ = item;
        false
    }

    fn is_suppressed(&self, node: &N, f: &JsFormatter) -> bool {
        f.context().comments().is_suppressed(node.syntax())
    }

    fn fmt_leading_comments(&self, node: &N, f: &mut JsFormatter) -> FormatResult<()> {
        format_leading_comments(node.syntax()).fmt(f)
    }

    fn fmt_dangling_comments(&self, node: &N, f: &mut JsFormatter) -> FormatResult<()> {
        format_dangling_comments(node.syntax())
            .with_soft_block_indent()
            .fmt(f)
    }

    fn fmt_trailing_comments(&self, node: &N, f: &mut JsFormatter) -> FormatResult<()> {
        format_trailing_comments(node.syntax()).fmt(f)
    }
}

pub trait FormatUnknownNodeRule<N>
where
    N: AstNode<Language = JsLanguage>,
{
    fn fmt(&self, node: &N, f: &mut JsFormatter) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(f)
    }
}

/// Format implementation specific to JavaScript tokens.
pub struct FormatJsSyntaxToken;

impl FormatRule<JsSyntaxToken> for FormatJsSyntaxToken {
    type Context = JsFormatContext;

    fn fmt(&self, token: &JsSyntaxToken, f: &mut JsFormatter) -> FormatResult<()> {
        f.state_mut().track_token(token);

        write!(
            f,
            [
                format_skipped_token_trivia(token),
                format_trimmed_token(token),
            ]
        )
    }
}

impl<'a> AsFormat<'a> for JsSyntaxToken {
    type Format = FormatRefWithRule<'a, JsSyntaxToken, FormatJsSyntaxToken>;

    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self, FormatJsSyntaxToken)
    }
}

impl IntoFormat<JsFormatContext> for JsSyntaxToken {
    type Format = FormatOwnedWithRule<JsSyntaxToken, FormatJsSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatJsSyntaxToken)
    }
}

struct JsFormatLanguage {
    options: JsFormatOptions,
}
impl JsFormatLanguage {
    fn new(options: JsFormatOptions) -> Self {
        Self { options }
    }
}

impl FormatLanguage for JsFormatLanguage {
    type SyntaxLanguage = JsLanguage;
    type Context = JsFormatContext;
    type CommentStyle = JsCommentStyle;
    type FormatRule = FormatJsSyntaxNode;

    fn transform(
        &self,
        root: &SyntaxNode<Self::SyntaxLanguage>,
    ) -> Option<(SyntaxNode<Self::SyntaxLanguage>, TransformSourceMap)> {
        Some(transform(root.clone()))
    }

    fn is_range_formatting_node(&self, node: &JsSyntaxNode) -> bool {
        let kind = node.kind();

        // Do not format variable declaration nodes, format the whole statement instead
        if matches!(kind, JsSyntaxKind::JS_VARIABLE_DECLARATION) {
            return false;
        }

        JsAnyStatement::can_cast(kind)
            || JsAnyDeclaration::can_cast(kind)
            || matches!(
                kind,
                JsSyntaxKind::JS_DIRECTIVE | JsSyntaxKind::JS_EXPORT | JsSyntaxKind::JS_IMPORT
            )
    }

    fn options(&self) -> &JsFormatOptions {
        &self.options
    }

    fn create_context(
        self,
        comments: Comments<Self::SyntaxLanguage>,
        source_map: Option<TransformSourceMap>,
    ) -> Self::Context {
        JsFormatContext::new(self.options, comments).with_source_map(source_map)
    }
}

/// Formats a range within a file, supported by Rome
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [JsFormatContext], which
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
    rome_formatter::format_range(root, range, JsFormatLanguage::new(options))
}

/// Formats a JavaScript (and its super languages) file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
pub fn format_node(
    options: JsFormatOptions,
    root: &JsSyntaxNode,
) -> FormatResult<Formatted<JsFormatContext>> {
    rome_formatter::format_node(root, JsFormatLanguage::new(options))
}

/// Formats a single node within a file, supported by Rome.
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [JsFormatContext], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Formatted] result
pub fn format_sub_tree(options: JsFormatOptions, root: &JsSyntaxNode) -> FormatResult<Printed> {
    rome_formatter::format_sub_tree(root, JsFormatLanguage::new(options))
}

#[cfg(test)]
mod tests {

    use super::{format_node, format_range};

    use crate::context::JsFormatOptions;
    use rome_formatter::IndentStyle;
    use rome_js_parser::{parse, parse_script};
    use rome_js_syntax::SourceType;
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
            JsFormatOptions::new(SourceType::js_script()).with_indent_style(IndentStyle::Space(4)),
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );

        let result = result.expect("range formatting failed");
        assert_eq!(
            result.as_code(),
            "function func() {\n        func(/* comment */);\n\n        let array = [1, 2];\n    }\n\n    function func2() {\n        const no_format = () => {};\n    }"
        );
        assert_eq!(
            result.range(),
            Some(TextRange::new(
                range_start - TextSize::from(56),
                range_end + TextSize::from(40)
            ))
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
            JsFormatOptions::new(SourceType::js_script()).with_indent_style(IndentStyle::Space(4)),
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );

        let result = result.expect("range formatting failed");
        // As a result of the indentation normalization, the number of spaces within
        // the object expression is currently rounded down from an odd indentation level
        assert_eq!(
            result.as_code(),
            "const veryLongIdentifierToCauseALineBreak = {\n            veryLongKeyToCauseALineBreak: \"veryLongValueToCauseALineBreak\",\n        };"
        );
        assert_eq!(
            result.range(),
            Some(TextRange::new(range_start, range_end + TextSize::from(1)))
        );
    }

    #[test]
    fn test_range_formatting_semicolon() {
        let input = "
    statement_1()
    statement_2()
    statement_3()
";

        let range_start = TextSize::try_from(input.find("statement_2").unwrap()).unwrap();
        let range_end = range_start + TextSize::of("statement_2()");

        let tree = parse_script(input, 0);
        let result = format_range(
            JsFormatOptions::new(SourceType::js_script()).with_indent_style(IndentStyle::Space(4)),
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );

        let result = result.expect("range formatting failed");
        assert_eq!(result.as_code(), "statement_2();");
        assert_eq!(result.range(), Some(TextRange::new(range_start, range_end)));
    }

    #[test]
    fn test_range_formatting_expression() {
        let input = "1 + 2 + 3 + 4 + 5";

        let range_start = TextSize::try_from(input.find("3 + 4").unwrap()).unwrap();
        let range_end = range_start + TextSize::of("3 + 4");

        let tree = parse_script(input, 0);
        let result = format_range(
            JsFormatOptions::new(SourceType::js_script()).with_indent_style(IndentStyle::Space(4)),
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );

        let result = result.expect("range formatting failed");
        assert_eq!(result.as_code(), "1 + 2 + 3 + 4 + 5;");
        assert_eq!(
            result.range(),
            Some(TextRange::new(TextSize::from(0), TextSize::of(input)))
        );
    }

    #[test]
    fn test_range_formatting_whitespace() {
        let input = "               ";

        let range_start = TextSize::from(5);
        let range_end = TextSize::from(5);

        let tree = parse_script(input, 0);
        let result = format_range(
            JsFormatOptions::new(SourceType::js_script()).with_indent_style(IndentStyle::Space(4)),
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );

        let result = result.expect("range formatting failed");
        assert_eq!(result.as_code(), "");
        assert_eq!(result.range(), Some(TextRange::new(range_start, range_end)));
    }

    #[ignore]
    #[test]
    // use this test check if your snippet prints as you wish, without using a snapshot
    fn quick_test() {
        let src = r#"
switch(x) {
  default: /* comment */
    break;
}
        "#;
        let syntax = SourceType::tsx();
        let tree = parse(src, 0, syntax);
        let options = JsFormatOptions::new(syntax);

        let result = format_node(options, &tree.syntax()).unwrap().print();
        // check_reformat(CheckReformatParams {
        //     root: &tree.syntax(),
        //     text: result.as_code(),
        //     source_type: syntax,
        //     file_name: "quick_test",
        //     options,
        // });
        assert_eq!(
            result.as_code(),
            r#"a;
loooooooooooooooooooooooooong7 =
	// rome-ignore format: test
		!     "looooooooooooooooooooooooooooooooooooooooooog";
"#
        );
    }

    #[test]
    fn format_range_out_of_bounds() {
        let src = "statement();";

        let syntax = SourceType::js_module();
        let tree = parse(src, 0, syntax);

        let result = format_range(
            JsFormatOptions::new(syntax),
            &tree.syntax(),
            TextRange::new(TextSize::from(0), TextSize::of(src) + TextSize::from(5)),
        );

        assert!(result.is_err());
    }
}
