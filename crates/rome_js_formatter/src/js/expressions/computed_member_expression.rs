use crate::prelude::*;

use crate::js::expressions::static_member_expression::member_chain_callee_needs_parens;
use crate::parentheses::NeedsParentheses;
use rome_formatter::{format_args, write};
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsComputedMemberAssignment,
    JsComputedMemberExpression, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
};
use rome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatJsComputedMemberExpression;

impl FormatNodeRule<JsComputedMemberExpression> for FormatJsComputedMemberExpression {
    fn fmt_fields(
        &self,
        node: &JsComputedMemberExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        JsAnyComputedMemberLike::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &JsComputedMemberExpression) -> bool {
        item.needs_parentheses()
    }
}

declare_node_union! {
    pub(crate) JsAnyComputedMemberLike = JsComputedMemberExpression | JsComputedMemberAssignment
}

impl Format<JsFormatContext> for JsAnyComputedMemberLike {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        write!(f, [self.object().format()])?;

        match self.member()? {
            JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsNumberLiteralExpression(literal),
            ) => {
                write!(
                    f,
                    [
                        self.optional_chain_token().format(),
                        self.l_brack_token().format(),
                        literal.format(),
                        self.r_brack_token().format()
                    ]
                )
            }
            member => {
                write![
                    f,
                    [group(&format_args![
                        self.optional_chain_token().format(),
                        self.l_brack_token().format(),
                        soft_line_break(),
                        soft_block_indent(&member.format()),
                        self.r_brack_token().format()
                    ]),]
                ]
            }
        }
    }
}

impl JsAnyComputedMemberLike {
    fn object(&self) -> SyntaxResult<JsAnyExpression> {
        match self {
            JsAnyComputedMemberLike::JsComputedMemberExpression(expression) => expression.object(),
            JsAnyComputedMemberLike::JsComputedMemberAssignment(assignment) => assignment.object(),
        }
    }

    fn l_brack_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyComputedMemberLike::JsComputedMemberExpression(expression) => {
                expression.l_brack_token()
            }
            JsAnyComputedMemberLike::JsComputedMemberAssignment(assignment) => {
                assignment.l_brack_token()
            }
        }
    }

    fn optional_chain_token(&self) -> Option<JsSyntaxToken> {
        match self {
            JsAnyComputedMemberLike::JsComputedMemberExpression(expression) => {
                expression.optional_chain_token()
            }
            JsAnyComputedMemberLike::JsComputedMemberAssignment(_) => None,
        }
    }

    fn member(&self) -> SyntaxResult<JsAnyExpression> {
        match self {
            JsAnyComputedMemberLike::JsComputedMemberExpression(expression) => expression.member(),
            JsAnyComputedMemberLike::JsComputedMemberAssignment(assignment) => assignment.member(),
        }
    }

    fn r_brack_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyComputedMemberLike::JsComputedMemberExpression(expression) => {
                expression.r_brack_token()
            }
            JsAnyComputedMemberLike::JsComputedMemberAssignment(assignment) => {
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
