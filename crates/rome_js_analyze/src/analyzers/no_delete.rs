use rome_analyze::{context::RuleContext, ActionCategory, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyAssignment, JsAnyAssignmentPattern, JsAnyExpression, JsAnyRoot,
    JsComputedMemberExpression, JsComputedMemberExpressionFields, JsStaticMemberExpression,
    JsStaticMemberExpressionFields, JsUnaryExpression, JsUnaryOperator, T,
};
use rome_rowan::{AstNode, AstNodeExt};

use crate::JsRuleAction;

pub(crate) enum NoDelete {}

impl Rule for NoDelete {
    const NAME: &'static str = "noDelete";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsUnaryExpression;
    type State = MemberExpression;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query_result();

        let op = node.operator().ok()?;
        if op != JsUnaryOperator::Delete {
            return None;
        }

        let argument = node.argument().ok()?;
        MemberExpression::try_from(argument).ok()
    }

    fn diagnostic(node: &Self::Query, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::warning(node.range(), markup! {
                "This is an unexpected use of the "<Emphasis>"delete"</Emphasis>" operator."
            })
            .summary("This is an unexpected use of the `delete` operator.\nReplace this expression with an `undefined` assignment")
        )
    }

    fn action(root: JsAnyRoot, node: &Self::Query, state: &Self::State) -> Option<JsRuleAction> {
        let root = root.replace_node(
            JsAnyExpression::from(node.clone()),
            JsAnyExpression::from(make::js_assignment_expression(
                state.clone().try_into().ok()?,
                make::token_decorated_with_space(T![=]),
                JsAnyExpression::from(make::js_identifier_expression(
                    make::js_reference_identifier(make::ident("undefined")),
                )),
            )),
        )?;

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Replace with undefined assignment" }.to_owned(),
            root,
        })
    }
}

/// Wrapper type for member expression nodes that can be safely converted to assignment patterns
#[derive(Clone)]
pub(crate) enum MemberExpression {
    JsStaticMemberExpression(JsStaticMemberExpression),
    JsComputedMemberExpression(JsComputedMemberExpression),
}

impl TryFrom<JsAnyExpression> for MemberExpression {
    type Error = ();

    fn try_from(value: JsAnyExpression) -> Result<Self, Self::Error> {
        match value {
            JsAnyExpression::JsStaticMemberExpression(expr) => {
                let JsStaticMemberExpressionFields {
                    object,
                    operator_token,
                    member,
                } = expr.as_fields();

                if object.is_err() || operator_token.is_err() || member.is_err() {
                    return Err(());
                }

                Ok(Self::JsStaticMemberExpression(expr))
            }
            JsAnyExpression::JsComputedMemberExpression(expr) => {
                let JsComputedMemberExpressionFields {
                    object,
                    optional_chain_token,
                    l_brack_token,
                    member,
                    r_brack_token,
                } = expr.as_fields();

                if object.is_err()
                    || optional_chain_token.is_some()
                    || l_brack_token.is_err()
                    || member.is_err()
                    || r_brack_token.is_err()
                {
                    return Err(());
                }

                Ok(Self::JsComputedMemberExpression(expr))
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<MemberExpression> for JsAnyAssignmentPattern {
    type Error = ();

    fn try_from(expr: MemberExpression) -> Result<Self, Self::Error> {
        match expr {
            MemberExpression::JsStaticMemberExpression(expr) => {
                let JsStaticMemberExpressionFields {
                    object,
                    operator_token,
                    member,
                } = expr.as_fields();

                Ok(Self::JsAnyAssignment(JsAnyAssignment::from(
                    make::js_static_member_assignment(
                        object.map_err(drop)?,
                        operator_token.map_err(drop)?,
                        member.map_err(drop)?,
                    ),
                )))
            }
            MemberExpression::JsComputedMemberExpression(expr) => {
                let JsComputedMemberExpressionFields {
                    object,
                    optional_chain_token: _,
                    l_brack_token,
                    member,
                    r_brack_token,
                } = expr.as_fields();

                Ok(Self::JsAnyAssignment(JsAnyAssignment::from(
                    make::js_computed_member_assignment(
                        object.map_err(drop)?,
                        l_brack_token.map_err(drop)?,
                        member.map_err(drop)?,
                        r_brack_token.map_err(drop)?,
                    ),
                )))
            }
        }
    }
}
