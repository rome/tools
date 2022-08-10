use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::write;
use rome_js_syntax::JsArrayExpressionFields;
use rome_js_syntax::{JsArrayExpression, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrayExpression;

impl FormatNodeRule<JsArrayExpression> for FormatJsArrayExpression {
    fn fmt_fields(&self, node: &JsArrayExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsArrayExpressionFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        let group_id = f.group_id("array");

        let elements = elements.format().with_options(Some(group_id));

        write!(
            f,
            [
                format_delimited(&l_brack_token?, &elements, &r_brack_token?)
                    .soft_block_indent_with_group_id(Some(group_id))
            ]
        )
    }
}

impl NeedsParentheses for JsArrayExpression {
    fn needs_parentheses(&self) -> bool {
        false
    }
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
