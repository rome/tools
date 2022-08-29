use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::write;
use rome_js_syntax::{JsSyntaxNode, TsTypeofType, TsTypeofTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeofType;

impl FormatNodeRule<TsTypeofType> for FormatTsTypeofType {
    fn fmt_fields(&self, node: &TsTypeofType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeofTypeFields {
            typeof_token,
            expression_name,
        } = node.as_fields();

        write![
            f,
            [typeof_token.format(), space(), expression_name.format()]
        ]
    }

    fn needs_parentheses(&self, item: &TsTypeofType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsTypeofType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
