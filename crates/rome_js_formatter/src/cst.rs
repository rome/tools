use crate::prelude::*;
use rome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::{AsFormat, IntoFormat, JsFormatContext};
use rome_js_syntax::{map_syntax_node, JsSyntaxNode};

pub struct FormatJsSyntaxNode;

impl rome_formatter::FormatRule<JsSyntaxNode> for FormatJsSyntaxNode {
    type Context = JsFormatContext;

    fn fmt(node: &JsSyntaxNode, f: &mut JsFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl<'a> AsFormat<'a> for JsSyntaxNode {
    type Format = FormatRefWithRule<'a, JsSyntaxNode, FormatJsSyntaxNode>;

    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}

impl IntoFormat<JsFormatContext> for JsSyntaxNode {
    type Format = FormatOwnedWithRule<JsSyntaxNode, FormatJsSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
