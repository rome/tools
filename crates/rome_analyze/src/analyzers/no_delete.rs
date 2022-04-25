use rome_console::markup;
use rome_diagnostics::Severity;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyAssignment, JsAnyAssignmentPattern, JsAnyExpression, JsAnyRoot,
    JsComputedMemberExpression, JsComputedMemberExpressionFields, JsStaticMemberExpression,
    JsStaticMemberExpressionFields, JsUnaryExpression, JsUnaryOperator, T,
};
use rome_rowan::{AstNode, AstNodeExt};

use crate::categories::ActionCategory;

use crate::registry::{Rule, RuleCodeFix, RuleDiagnostic};

pub(crate) enum NoDelete {}

impl Rule for NoDelete {
    const NAME: &'static str = "noDelete";
    const ACTION_CATEGORIES: &'static [ActionCategory] = &[];

    type Query = JsUnaryExpression;
    type State = MemberExpression;

    fn run(node: &Self::Query) -> Option<Self::State> {
        let op = node.operator().ok()?;
        if op != JsUnaryOperator::Delete {
            return None;
        }

        let argument = node.argument().ok()?;
        MemberExpression::try_from(argument).ok()
    }

    fn diagnostic(node: &Self::Query, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic {
            severity: Severity::Warning,
            range: node.syntax().text_trimmed_range(),
            message: markup! {
                "This is an unexpected use of the "<Emphasis>"delete"</Emphasis>" operator."
            }
            .to_owned(),
        })
    }

    fn code_fix(root: JsAnyRoot, node: &Self::Query, state: &Self::State) -> Option<RuleCodeFix> {
        let root = root.replace_node_retain_trivia(
            JsAnyExpression::from(node.clone()),
            JsAnyExpression::from(make::js_assignment_expression(
                state.clone().try_into().ok()?,
                make::token_decorated_with_space(T![=]),
                JsAnyExpression::from(make::js_identifier_expression(
                    make::js_reference_identifier(make::ident("undefined")),
                )),
            )),
        )?;

        Some(RuleCodeFix { root })
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
