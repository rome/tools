use crate::parentheses::NeedsParentheses;
use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::JsArrayAssignmentPattern;
use rome_js_syntax::{JsArrayAssignmentPatternFields, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrayAssignmentPattern;

impl FormatNodeRule<JsArrayAssignmentPattern> for FormatJsArrayAssignmentPattern {
    fn fmt_fields(&self, node: &JsArrayAssignmentPattern, f: &mut JsFormatter) -> FormatResult<()> {
        let JsArrayAssignmentPatternFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_delimited(&l_brack_token?, &elements.format(), &r_brack_token?,)
                    .soft_block_indent()
            ]
        )
    }

    fn needs_parentheses(&self, item: &JsArrayAssignmentPattern) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsArrayAssignmentPattern {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        false
    }
}
