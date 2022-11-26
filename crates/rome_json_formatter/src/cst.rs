use crate::prelude::*;
use rome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};
use rome_json_syntax::{map_syntax_node, JsonSyntaxNode};

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatJsonSyntaxNode;

impl FormatRule<JsonSyntaxNode> for FormatJsonSyntaxNode {
    type Context = JsonFormatContext;

    fn fmt(&self, node: &JsonSyntaxNode, f: &mut JsonFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<JsonFormatContext> for JsonSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, JsonSyntaxNode, FormatJsonSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatJsonSyntaxNode)
    }
}

impl IntoFormat<JsonFormatContext> for JsonSyntaxNode {
    type Format = FormatOwnedWithRule<JsonSyntaxNode, FormatJsonSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatJsonSyntaxNode)
    }
}
