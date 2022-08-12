use crate::prelude::*;

use crate::parentheses::{
    is_binary_like_left_or_right, is_callee, is_member_object, is_spread, is_tag, NeedsParentheses,
};
use rome_formatter::write;
use rome_js_syntax::TsAsExpressionFields;
use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode, TsAsExpression};

#[derive(Debug, Clone, Default)]
pub struct FormatTsAsExpression;

impl FormatNodeRule<TsAsExpression> for FormatTsAsExpression {
    fn fmt_fields(&self, node: &TsAsExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAsExpressionFields {
            ty,
            as_token,
            expression,
        } = node.as_fields();

        write![
            f,
            [
                expression.format(),
                space(),
                as_token.format(),
                space(),
                ty.format(),
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsAsExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsAsExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match parent.kind() {
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION
            | JsSyntaxKind::JS_EXTENDS_CLAUSE
            | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
            | JsSyntaxKind::JS_UNARY_EXPRESSION
            | JsSyntaxKind::JS_AWAIT_EXPRESSION
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => true,

            _ => {
                is_callee(self.syntax(), parent)
                    || is_tag(self.syntax(), parent)
                    || is_spread(self.syntax(), parent)
                    || is_member_object(self.syntax(), parent)
                    || is_binary_like_left_or_right(self.syntax(), parent)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::{SourceType, TsAsExpression};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("5 as number ? true : false", TsAsExpression);
        assert_needs_parentheses!("cond ? x as number : false", TsAsExpression);
        assert_needs_parentheses!("cond ? true : x as number", TsAsExpression);

        assert_needs_parentheses!("class X extends (B as number) {}", TsAsExpression);

        assert_needs_parentheses!("(x as Function)()", TsAsExpression);
        assert_needs_parentheses!("(x as Function)?.()", TsAsExpression);
        assert_needs_parentheses!("new (x as Function)()", TsAsExpression);

        assert_needs_parentheses!("<number>(x as any)", TsAsExpression);
        assert_needs_parentheses!("(x as any)`template`", TsAsExpression);
        assert_needs_parentheses!("!(x as any)", TsAsExpression);
        assert_needs_parentheses!("[...(x as any)]", TsAsExpression);
        assert_needs_parentheses!("({...(x as any)})", TsAsExpression);
        assert_needs_parentheses!(
            "<test {...(x as any)} />",
            TsAsExpression,
            SourceType::tsx()
        );
        assert_needs_parentheses!(
            "<test>{...(x as any)}</test>",
            TsAsExpression,
            SourceType::tsx()
        );
        assert_needs_parentheses!("await (x as any)", TsAsExpression);
        assert_needs_parentheses!("(x as any)!", TsAsExpression);

        assert_needs_parentheses!("(x as any).member", TsAsExpression);
        assert_needs_parentheses!("(x as any)[member]", TsAsExpression);
        assert_not_needs_parentheses!("object[x as any]", TsAsExpression);

        assert_needs_parentheses!("(x as any) + (y as any)", TsAsExpression[0]);
        assert_needs_parentheses!("(x as any) + (y as any)", TsAsExpression[1]);

        assert_needs_parentheses!("(x as any) && (y as any)", TsAsExpression[0]);
        assert_needs_parentheses!("(x as any) && (y as any)", TsAsExpression[1]);

        assert_needs_parentheses!("(x as any) in (y as any)", TsAsExpression[0]);
        assert_needs_parentheses!("(x as any) in (y as any)", TsAsExpression[1]);

        assert_needs_parentheses!("(x as any) instanceof (y as any)", TsAsExpression[0]);
        assert_needs_parentheses!("(x as any) instanceof (y as any)", TsAsExpression[1]);

        assert_not_needs_parentheses!("x as number as string", TsAsExpression[1]);
    }
}
