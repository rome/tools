use crate::parentheses::NeedsParentheses;
use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::JsArrayAssignmentPattern;
use rome_js_syntax::{JsArrayAssignmentPatternFields, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsArrayAssignmentPattern;

impl FormatNodeRule<JsArrayAssignmentPattern> for FormatJsArrayAssignmentPattern {
    fn fmt_fields(&self, node: &JsArrayAssignmentPattern, f: &mut JsFormatter) -> FormatResult<()> {
        let JsArrayAssignmentPatternFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        write!(f, [l_brack_token.format(),])?;

        if elements.is_empty() {
            write!(
                f,
                [format_dangling_comments(node.syntax()).with_block_indent()]
            )?;
        } else {
            write!(f, [group(&soft_block_indent(&elements.format()))])?;
        }

        write!(f, [r_brack_token.format()])
    }

    fn needs_parentheses(&self, item: &JsArrayAssignmentPattern) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsArrayAssignmentPattern,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled inside of `fmt_fields`
        Ok(())
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
