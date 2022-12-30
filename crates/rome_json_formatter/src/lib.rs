pub mod context;
mod cst;
mod format_string;
mod generated;
mod json;
mod prelude;
mod separated;

pub(crate) use crate::context::JsonFormatContext;
use crate::context::JsonFormatOptions;
use crate::cst::FormatJsonSyntaxNode;
use rome_formatter::comments::{CommentKind, CommentStyle};
use rome_formatter::prelude::*;
use rome_formatter::{
    FormatContext, FormatLanguage, FormatOwnedWithRule, FormatRefWithRule, FormatToken,
    TransformSourceMap,
};
use rome_formatter::{Formatted, Printed};
use rome_json_syntax::{AnyJsonValue, JsonLanguage, JsonSyntaxNode, JsonSyntaxToken};
use rome_rowan::{AstNode, SyntaxNode, SyntaxTriviaPieceComments, TextRange};

include!("../../rome_formatter/shared_traits.rs");

pub(crate) type JsonFormatter<'buf> = Formatter<'buf, JsonFormatContext>;

pub(crate) trait FormatNodeRule<N>
where
    N: AstNode<Language = JsonLanguage>,
{
    fn fmt(&self, node: &N, f: &mut JsonFormatter) -> FormatResult<()> {
        f.comments().mark_suppression_checked(node.syntax());

        self.fmt_fields(node, f)
    }

    fn fmt_fields(&self, node: &N, f: &mut JsonFormatter) -> FormatResult<()>;
}

/// Rule for formatting an bogus nodes.
pub(crate) trait FormatBogusNodeRule<N>
where
    N: AstNode<Language = JsonLanguage>,
{
    fn fmt(&self, node: &N, f: &mut JsonFormatter) -> FormatResult<()> {
        format_bogus_node(node.syntax()).fmt(f)
    }
}

#[derive(Debug, Default, Clone)]
pub struct JsonFormatLanguage {
    options: JsonFormatOptions,
}

impl JsonFormatLanguage {
    pub fn new(options: JsonFormatOptions) -> Self {
        Self { options }
    }
}

impl FormatLanguage for JsonFormatLanguage {
    type SyntaxLanguage = JsonLanguage;
    type Context = JsonFormatContext;
    type FormatRule = FormatJsonSyntaxNode;

    fn is_range_formatting_node(&self, node: &SyntaxNode<Self::SyntaxLanguage>) -> bool {
        AnyJsonValue::can_cast(node.kind())
    }

    fn options(&self) -> &<Self::Context as FormatContext>::Options {
        &self.options
    }

    fn create_context(
        self,
        _root: &JsonSyntaxNode,
        _source_map: Option<TransformSourceMap>,
    ) -> Self::Context {
        JsonFormatContext::new(self.options)
    }
}

#[derive(Default)]
pub struct JsonCommentStyle;

impl CommentStyle for JsonCommentStyle {
    type Language = JsonLanguage;

    fn get_comment_kind(_: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
        CommentKind::Line
    }
}

/// Format implementation specific to JavaScript tokens.
pub(crate) type FormatJsonSyntaxToken = FormatToken<JsonFormatContext>;

impl AsFormat<JsonFormatContext> for JsonSyntaxToken {
    type Format<'a> = FormatRefWithRule<'a, JsonSyntaxToken, FormatJsonSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatJsonSyntaxToken::default())
    }
}

impl IntoFormat<JsonFormatContext> for JsonSyntaxToken {
    type Format = FormatOwnedWithRule<JsonSyntaxToken, FormatJsonSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatJsonSyntaxToken::default())
    }
}

/// Formats a range within a file, supported by Rome
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [JsonFormatOptions], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Printed] result with a range corresponding to the
/// range of the input that was effectively overwritten by the formatter
pub fn format_range(
    options: JsonFormatOptions,
    root: &JsonSyntaxNode,
    range: TextRange,
) -> FormatResult<Printed> {
    rome_formatter::format_range(root, range, JsonFormatLanguage::new(options))
}

/// Formats a JSON syntax tree.
///
/// It returns the [Formatted] document that can be printed to a string.
pub fn format_node(
    options: JsonFormatOptions,
    root: &JsonSyntaxNode,
) -> FormatResult<Formatted<JsonFormatContext>> {
    rome_formatter::format_node(root, JsonFormatLanguage::new(options))
}

/// Formats a single node within a file, supported by Rome.
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [JsonFormatOptions], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// Returns the [Printed] code.
pub fn format_sub_tree(options: JsonFormatOptions, root: &JsonSyntaxNode) -> FormatResult<Printed> {
    rome_formatter::format_sub_tree(root, JsonFormatLanguage::new(options))
}

#[cfg(test)]
mod tests {
    use crate::context::JsonFormatOptions;
    use crate::format_node;
    use rome_diagnostics::FileId;
    use rome_json_parser::parse_json;

    #[test]
    fn smoke_test() {
        let src = r#"
{
    "a": 5,
    "b": [1, 2, 3, 4],
    "c": null,
    "d": true,
    "e": false
}
"#;
        let parse = parse_json(src, FileId::zero());
        let options = JsonFormatOptions::default();
        let formatted = format_node(options, &parse.syntax()).unwrap();
        assert_eq!(
            formatted.print().unwrap().as_code(),
            "{\n\t\"a\": 5,\n\t\"b\": [1, 2, 3, 4],\n\t\"c\": null,\n\t\"d\": true,\n\t\"e\": false\n}\n"
        );
    }
}
