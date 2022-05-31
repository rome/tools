use crate::prelude::*;
use rome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::{AsFormat, IntoFormat, JsFormatContext};
use rome_js_syntax::{map_syntax_node, JsSyntaxNode};

pub struct FormatJsSyntaxNode;

impl rome_formatter::FormatRule<JsSyntaxNode> for FormatJsSyntaxNode {
    type Context = JsFormatContext;

    fn format(node: &JsSyntaxNode, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        map_syntax_node!(node.clone(), node => formatted![formatter, [node.format()]])
    }
}

impl<'a> AsFormat<'a> for JsSyntaxNode {
    type Format = FormatRefWithRule<'a, JsSyntaxNode, FormatJsSyntaxNode>;

    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}

impl IntoFormat for JsSyntaxNode {
    type Format = FormatOwnedWithRule<JsSyntaxNode, FormatJsSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
