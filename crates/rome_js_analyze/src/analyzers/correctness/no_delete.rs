use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyAssignment, JsAnyAssignmentPattern, JsAnyExpression, JsComputedMemberExpression,
    JsComputedMemberExpressionFields, JsStaticMemberExpression, JsStaticMemberExpressionFields,
    JsUnaryExpression, JsUnaryOperator, T,
};
use rome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow the use of the `delete` operator
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const arr = [['a','b','c'], [1, 2, 3]];
    /// delete arr[0][2];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const obj = {a: {b: {c: 123}}};
    /// delete obj.a.b.c;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = new Set([1,2,3]);
    /// foo.delete(1);
    ///```
    pub(crate) NoDelete {
        version: "0.7.0",
        name: "noDelete",
        recommended: true,
    }
}

impl Rule for NoDelete {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsUnaryExpression>;
    type State = MemberExpression;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        let op = node.operator().ok()?;
        if op != JsUnaryOperator::Delete {
            return None;
        }

        let argument = node.argument().ok()?;
        MemberExpression::try_from(argument).ok()
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(rule_category!(),node.range(), markup! {
                "This is an unexpected use of the "<Emphasis>"delete"</Emphasis>" operator."
            })
            .summary("This is an unexpected use of the `delete` operator.\nReplace this expression with an `undefined` assignment")
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            JsAnyExpression::from(node.clone()),
            JsAnyExpression::from(make::js_assignment_expression(
                state.clone().try_into().ok()?,
                make::token_decorated_with_space(T![=]),
                JsAnyExpression::from(make::js_identifier_expression(
                    make::js_reference_identifier(make::ident("undefined")),
                )),
            )),
        );

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Replace with undefined assignment" }.to_owned(),
            mutation,
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

                match object {
                    Ok(expr) if has_optional(&expr)? => return Err(()),
                    Err(_) => return Err(()),
                    _ => {}
                }

                match operator_token {
                    Ok(token) if token.kind() == T![?.] => return Err(()),
                    Err(_) => return Err(()),
                    _ => {}
                }

                if member.is_err() {
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

                match object {
                    Ok(expr) if has_optional(&expr)? => return Err(()),
                    Err(_) => return Err(()),
                    _ => {}
                }

                if optional_chain_token.is_some()
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

fn has_optional(expr: &JsAnyExpression) -> Result<bool, ()> {
    match expr {
        JsAnyExpression::JsStaticMemberExpression(expr) => match expr.operator_token() {
            Ok(token) if token.kind() == T![?.] => Ok(true),
            Err(_) => Err(()),
            _ => match expr.object() {
                Ok(expr) => has_optional(&expr),
                Err(_) => Err(()),
            },
        },
        JsAnyExpression::JsComputedMemberExpression(expr) => match expr.optional_chain_token() {
            Some(_) => Ok(true),
            None => match expr.object() {
                Ok(expr) => has_optional(&expr),
                Err(_) => Err(()),
            },
        },
        _ => Ok(false),
    }
}
