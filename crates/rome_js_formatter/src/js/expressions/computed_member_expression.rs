use crate::prelude::*;

use crate::js::expressions::static_member_expression::member_chain_callee_needs_parens;
use crate::parentheses::NeedsParentheses;
use rome_formatter::{format_args, write};
use rome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsComputedMemberAssignment,
    JsComputedMemberExpression, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
};
use rome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsComputedMemberExpression;

impl FormatNodeRule<JsComputedMemberExpression> for FormatJsComputedMemberExpression {
    fn fmt_fields(
        &self,
        node: &JsComputedMemberExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        AnyJsComputedMemberLike::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &JsComputedMemberExpression) -> bool {
        item.needs_parentheses()
    }
}

declare_node_union! {
    pub(crate) AnyJsComputedMemberLike = JsComputedMemberExpression | JsComputedMemberAssignment
}

impl Format<JsFormatContext> for AnyJsComputedMemberLike {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        write!(f, [self.object().format()])?;

        FormatComputedMemberLookup(self).fmt(f)
    }
}

/// Formats the lookup portion (everything except the object) of a computed member like.
pub(crate) struct FormatComputedMemberLookup<'a>(&'a AnyJsComputedMemberLike);

impl<'a> FormatComputedMemberLookup<'a> {
    pub(crate) fn new(member_like: &'a AnyJsComputedMemberLike) -> Self {
        Self(member_like)
    }
}

impl Format<JsFormatContext> for FormatComputedMemberLookup<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self.0.member()? {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(literal),
            ) => {
                write!(
                    f,
                    [
                        self.0.optional_chain_token().format(),
                        self.0.l_brack_token().format(),
                        literal.format(),
                        self.0.r_brack_token().format()
                    ]
                )
            }
            member => {
                write![
                    f,
                    [group(&format_args![
                        self.0.optional_chain_token().format(),
                        self.0.l_brack_token().format(),
                        soft_block_indent(&member.format()),
                        self.0.r_brack_token().format()
                    ])]
                ]
            }
        }
    }
}

impl AnyJsComputedMemberLike {
    fn object(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            AnyJsComputedMemberLike::JsComputedMemberExpression(expression) => expression.object(),
            AnyJsComputedMemberLike::JsComputedMemberAssignment(assignment) => assignment.object(),
        }
    }

    fn l_brack_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsComputedMemberLike::JsComputedMemberExpression(expression) => {
                expression.l_brack_token()
            }
            AnyJsComputedMemberLike::JsComputedMemberAssignment(assignment) => {
                assignment.l_brack_token()
            }
        }
    }

    fn optional_chain_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsComputedMemberLike::JsComputedMemberExpression(expression) => {
                expression.optional_chain_token()
            }
            AnyJsComputedMemberLike::JsComputedMemberAssignment(_) => None,
        }
    }

    fn member(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            AnyJsComputedMemberLike::JsComputedMemberExpression(expression) => expression.member(),
            AnyJsComputedMemberLike::JsComputedMemberAssignment(assignment) => assignment.member(),
        }
    }

    fn r_brack_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsComputedMemberLike::JsComputedMemberExpression(expression) => {
                expression.r_brack_token()
            }
            AnyJsComputedMemberLike::JsComputedMemberAssignment(assignment) => {
                assignment.r_brack_token()
            }
        }
    }
}

impl NeedsParentheses for JsComputedMemberExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        if self.is_optional_chain() && matches!(parent.kind(), JsSyntaxKind::JS_NEW_EXPRESSION) {
            return true;
        }

        member_chain_callee_needs_parens(self.clone().into(), parent)
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::JsComputedMemberExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("new (test()[a])()", JsComputedMemberExpression);
        assert_needs_parentheses!("new (test().a[b])()", JsComputedMemberExpression);
        assert_needs_parentheses!(
            "new (test()`template`[index])()",
            JsComputedMemberExpression
        );
        assert_needs_parentheses!("new (test()![member])()", JsComputedMemberExpression);

        assert_needs_parentheses!("new (a?.b[c])()", JsComputedMemberExpression);
        assert_not_needs_parentheses!("new (test[a])()", JsComputedMemberExpression);
    }
}
