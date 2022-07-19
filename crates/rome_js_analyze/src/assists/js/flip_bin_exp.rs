use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsBinaryExpression, JsBinaryExpressionFields, JsBinaryOperator, JsSyntaxKind, T,
};
use rome_rowan::AstNodeExt;

use crate::JsRuleAction;

declare_rule! {
    "0.7.0",
    /// Provides a refactor to invert the left and right hand side of a binary expression
    ///
    /// ## Examples
    ///
    /// ```js
    /// (a < b)
    /// ```
    pub(crate) FlipBinExp = "flipBinExp"
}

impl Rule for FlipBinExp {
    const CATEGORY: RuleCategory = RuleCategory::Action;

    type Query = Ast<JsBinaryExpression>;
    type State = JsSyntaxKind;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        let JsBinaryExpressionFields {
            left,
            operator_token: _,
            right,
        } = node.as_fields();

        // Ensure the node doesn't have any syntax error
        left.ok()?;
        right.ok()?;

        invert_op(node.operator().ok()?)
    }

    fn action(ctx: &RuleContext<Self>, op: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        let prev_left = node.left().ok()?;
        let new_left = node.right().ok()?;
        let new_node = node.clone().replace_node(prev_left, new_left)?;

        let prev_op = new_node.operator_token().ok()?;
        let new_op = make::token(*op);
        let new_node = new_node.replace_token(prev_op, new_op)?;

        let prev_right = new_node.right().ok()?;
        let new_right = node.left().ok()?;
        let new_node = new_node.replace_node(prev_right, new_right)?;

        Some(JsRuleAction {
            category: ActionCategory::Refactor,
            applicability: Applicability::Always,
            message: markup! { "Flip Binary Expression" }.to_owned(),
            root: ctx.root().replace_node(node.clone(), new_node)?,
        })
    }
}

fn invert_op(op: JsBinaryOperator) -> Option<JsSyntaxKind> {
    match op {
        JsBinaryOperator::LessThan => Some(T![>]),
        JsBinaryOperator::GreaterThan => Some(T![<]),
        JsBinaryOperator::LessThanOrEqual => Some(T![>=]),
        JsBinaryOperator::GreaterThanOrEqual => Some(T![<=]),
        JsBinaryOperator::Equality => Some(T![==]),
        JsBinaryOperator::StrictEquality => Some(T![===]),
        JsBinaryOperator::Inequality => Some(T![!=]),
        JsBinaryOperator::StrictInequality => Some(T![!==]),
        JsBinaryOperator::Plus => Some(T![+]),
        JsBinaryOperator::Minus => None,
        JsBinaryOperator::Times => Some(T![*]),
        JsBinaryOperator::Divide => None,
        JsBinaryOperator::Remainder => None,
        JsBinaryOperator::Exponent => None,
        JsBinaryOperator::LeftShift => None,
        JsBinaryOperator::RightShift => None,
        JsBinaryOperator::UnsignedRightShift => None,
        JsBinaryOperator::BitwiseAnd => Some(T![&]),
        JsBinaryOperator::BitwiseOr => Some(T![|]),
        JsBinaryOperator::BitwiseXor => Some(T![^]),
    }
}
