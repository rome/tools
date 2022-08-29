use crate::prelude::*;

use crate::parentheses::{is_callee, is_member_object, is_spread, is_tag, NeedsParentheses};
use rome_formatter::write;
use rome_js_syntax::JsSyntaxNode;
use rome_js_syntax::{JsSyntaxKind, TsTypeAssertionExpression, TsTypeAssertionExpressionFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeAssertionExpression;

impl FormatNodeRule<TsTypeAssertionExpression> for FormatTsTypeAssertionExpression {
    fn fmt_fields(
        &self,
        node: &TsTypeAssertionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsTypeAssertionExpressionFields {
            l_angle_token,
            ty,
            r_angle_token,
            expression,
        } = node.as_fields();

        write![
            f,
            [
                format_delimited(&l_angle_token?, &ty.format(), &r_angle_token?,)
                    .soft_block_indent(),
                expression.format()
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsTypeAssertionExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsTypeAssertionExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match parent.kind() {
            JsSyntaxKind::TS_AS_EXPRESSION => true,
            _ => type_cast_like_needs_parens(self.syntax(), parent),
        }
    }
}

pub(super) fn type_cast_like_needs_parens(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert!(matches!(
        node.kind(),
        JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION | JsSyntaxKind::TS_AS_EXPRESSION
    ));

    match parent.kind() {
        JsSyntaxKind::JS_EXTENDS_CLAUSE
        | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
        | JsSyntaxKind::JS_UNARY_EXPRESSION
        | JsSyntaxKind::JS_AWAIT_EXPRESSION
        | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => true,

        _ => {
            is_callee(node, parent)
                || is_tag(node, parent)
                || is_spread(node, parent)
                || is_member_object(node, parent)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::TsTypeAssertionExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("(<number> x) as any", TsTypeAssertionExpression);

        assert_needs_parentheses!("class X extends (<number>B) {}", TsTypeAssertionExpression);

        assert_needs_parentheses!("(<Function>x)()", TsTypeAssertionExpression);
        assert_needs_parentheses!("(<Function>x)?.()", TsTypeAssertionExpression);
        assert_needs_parentheses!("new (<Function>x)()", TsTypeAssertionExpression);

        assert_needs_parentheses!("<number>(<any>x)", TsTypeAssertionExpression[1]);
        assert_needs_parentheses!("<number>(<any>x)", TsTypeAssertionExpression[1]);
        assert_needs_parentheses!("(<any>x)`template`", TsTypeAssertionExpression);
        assert_needs_parentheses!("!(<any>x)", TsTypeAssertionExpression);
        assert_needs_parentheses!("[...(<any>x)]", TsTypeAssertionExpression);
        assert_needs_parentheses!("({...(<any>x)})", TsTypeAssertionExpression);

        assert_needs_parentheses!("await (<any>x)", TsTypeAssertionExpression);
        assert_needs_parentheses!("(<any>x)!", TsTypeAssertionExpression);

        assert_needs_parentheses!("(<any>x).member", TsTypeAssertionExpression);
        assert_needs_parentheses!("(<any>x)[member]", TsTypeAssertionExpression);
        assert_not_needs_parentheses!("object[<any>x]", TsTypeAssertionExpression);
    }
}
