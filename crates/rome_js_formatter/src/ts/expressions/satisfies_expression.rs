use crate::prelude::*;

use crate::parentheses::{
    is_binary_like_left_or_right, is_callee, is_member_object, NeedsParentheses,
};
use crate::ts::expressions::type_assertion_expression::type_cast_like_needs_parens;
use rome_formatter::write;
use rome_js_syntax::TsSatisfiesExpressionFields;
use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode, TsSatisfiesExpression};

#[derive(Debug, Clone, Default)]
pub struct FormatTsSatisfiesExpression;

impl FormatNodeRule<TsSatisfiesExpression> for FormatTsSatisfiesExpression {
    fn fmt_fields(&self, node: &TsSatisfiesExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let TsSatisfiesExpressionFields {
            ty,
            satisfies_token,
            expression,
        } = node.as_fields();

        let format_inner = format_with(|f| {
            write![
                f,
                [
                    expression.format(),
                    space(),
                    satisfies_token.format(),
                    space(),
                    ty.format(),
                ]
            ]
        });

        let parent = node.syntax().parent();

        let is_callee_or_object = parent.map_or(false, |parent| {
            is_callee(node.syntax(), &parent) || is_member_object(node.syntax(), &parent)
        });

        if is_callee_or_object {
            write!(f, [group(&soft_block_indent(&format_inner))])
        } else {
            write!(f, [format_inner])
        }
    }

    fn needs_parentheses(&self, item: &TsSatisfiesExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsSatisfiesExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match parent.kind() {
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => true,

            _ => {
                type_cast_like_needs_parens(self.syntax(), parent)
                    || is_binary_like_left_or_right(self.syntax(), parent)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::{SourceType, TsSatisfiesExpression};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("5 satisfies number ? true : false", TsSatisfiesExpression);
        assert_needs_parentheses!("cond ? x satisfies number : false", TsSatisfiesExpression);
        assert_needs_parentheses!("cond ? true : x satisfies number", TsSatisfiesExpression);

        assert_needs_parentheses!(
            "class X extends (B satisfies number) {}",
            TsSatisfiesExpression
        );

        assert_needs_parentheses!("(x satisfies Function)()", TsSatisfiesExpression);
        assert_needs_parentheses!("(x satisfies Function)?.()", TsSatisfiesExpression);
        assert_needs_parentheses!("new (x satisfies Function)()", TsSatisfiesExpression);

        assert_needs_parentheses!("<number>(x satisfies any)", TsSatisfiesExpression);
        assert_needs_parentheses!("(x satisfies any)`template`", TsSatisfiesExpression);
        assert_needs_parentheses!("!(x satisfies any)", TsSatisfiesExpression);
        assert_needs_parentheses!("[...(x satisfies any)]", TsSatisfiesExpression);
        assert_needs_parentheses!("({...(x satisfies any)})", TsSatisfiesExpression);
        assert_needs_parentheses!(
            "<test {...(x satisfies any)} />",
            TsSatisfiesExpression,
            SourceType::tsx()
        );
        assert_needs_parentheses!(
            "<test>{...(x satisfies any)}</test>",
            TsSatisfiesExpression,
            SourceType::tsx()
        );
        assert_needs_parentheses!("await (x satisfies any)", TsSatisfiesExpression);
        assert_needs_parentheses!("(x satisfies any)!", TsSatisfiesExpression);

        assert_needs_parentheses!("(x satisfies any).member", TsSatisfiesExpression);
        assert_needs_parentheses!("(x satisfies any)[member]", TsSatisfiesExpression);
        assert_not_needs_parentheses!("object[x satisfies any]", TsSatisfiesExpression);

        assert_needs_parentheses!(
            "(x satisfies any) + (y satisfies any)",
            TsSatisfiesExpression[0]
        );
        assert_needs_parentheses!(
            "(x satisfies any) + (y satisfies any)",
            TsSatisfiesExpression[1]
        );

        assert_needs_parentheses!(
            "(x satisfies any) && (y satisfies any)",
            TsSatisfiesExpression[0]
        );
        assert_needs_parentheses!(
            "(x satisfies any) && (y satisfies any)",
            TsSatisfiesExpression[1]
        );

        assert_needs_parentheses!(
            "(x satisfies any) in (y satisfies any)",
            TsSatisfiesExpression[0]
        );
        assert_needs_parentheses!(
            "(x satisfies any) in (y satisfies any)",
            TsSatisfiesExpression[1]
        );

        assert_needs_parentheses!(
            "(x satisfies any) instanceof (y satisfies any)",
            TsSatisfiesExpression[0]
        );
        assert_needs_parentheses!(
            "(x satisfies any) instanceof (y satisfies any)",
            TsSatisfiesExpression[1]
        );

        assert_not_needs_parentheses!(
            "x satisfies number satisfies string",
            TsSatisfiesExpression[1]
        );
    }
}
