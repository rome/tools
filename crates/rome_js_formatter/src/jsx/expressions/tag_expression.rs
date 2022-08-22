use crate::parentheses::{is_callee, is_tag, NeedsParentheses};
use crate::prelude::*;
use crate::utils::jsx::{get_wrap_state, WrapState};
use rome_formatter::{format_args, write};
use rome_js_syntax::{
    JsBinaryExpression, JsBinaryOperator, JsSyntaxKind, JsSyntaxNode, JsxTagExpression,
};
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxTagExpression;

impl FormatNodeRule<JsxTagExpression> for FormatJsxTagExpression {
    fn fmt_fields(&self, node: &JsxTagExpression, f: &mut JsFormatter) -> FormatResult<()> {
        match get_wrap_state(node) {
            WrapState::WrapOnBreak => write![
                f,
                [group(&format_args![
                    if_group_breaks(&text("(")),
                    soft_block_indent(&format_args![node.tag().format()]),
                    if_group_breaks(&text(")"))
                ])]
            ],
            WrapState::NoWrap => write![f, [node.tag().format()]],
        }
    }

    fn needs_parentheses(&self, item: &JsxTagExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsxTagExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match parent.kind() {
            JsSyntaxKind::JS_BINARY_EXPRESSION => {
                let binary = JsBinaryExpression::unwrap_cast(parent.clone());

                let is_left = binary.left().map(AstNode::into_syntax).as_ref() == Ok(self.syntax());
                matches!(binary.operator(), Ok(JsBinaryOperator::LessThan)) && is_left
            }
            JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::JS_AWAIT_EXPRESSION
            | JsSyntaxKind::JS_EXTENDS_CLAUSE
            | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_SEQUENCE_EXPRESSION
            | JsSyntaxKind::JS_UNARY_EXPRESSION
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
            | JsSyntaxKind::JS_SPREAD
            | JsSyntaxKind::JSX_SPREAD_ATTRIBUTE
            | JsSyntaxKind::JSX_SPREAD_CHILD => true,
            _ => is_callee(self.syntax(), parent) || is_tag(self.syntax(), parent),
        }
    }
}
