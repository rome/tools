use crate::prelude::*;
use rome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::{AsFormat, IntoFormat, JsFormatContext};
use rome_js_syntax::{map_syntax_node, JsSyntaxNode};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatJsSyntaxNode;

impl rome_formatter::FormatRule<JsSyntaxNode> for FormatJsSyntaxNode {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsSyntaxNode, f: &mut JsFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat for JsSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, JsSyntaxNode, FormatJsSyntaxNode>;

    fn format<'a>(&'a self) -> Self::Format<'a> {
        FormatRefWithRule::new(self, FormatJsSyntaxNode)
    }
}

impl IntoFormat<JsFormatContext> for JsSyntaxNode {
    type Format = FormatOwnedWithRule<JsSyntaxNode, FormatJsSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatJsSyntaxNode)
    }
}
