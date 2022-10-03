use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::write;
use rome_js_syntax::{JsSyntaxNode, TsBigIntLiteralType, TsBigIntLiteralTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsBigIntLiteralType;

impl FormatNodeRule<TsBigIntLiteralType> for FormatTsBigIntLiteralType {
    fn fmt_fields(&self, node: &TsBigIntLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsBigIntLiteralTypeFields {
            minus_token,
            literal_token,
        } = node.as_fields();

        write![f, [minus_token.format(), literal_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsBigIntLiteralType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsBigIntLiteralType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
