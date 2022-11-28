use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{JsAnyExpression, JsSyntaxKind, TsNonNullAssertionExpression};
use rome_rowan::{AstNode, BatchMutationExt, SyntaxNodeCast};

declare_rule! {
    /// Disallow non-null assertions using the `!` postfix operator.
    ///
    /// TypeScript's `!` non-null assertion operator asserts to the type system that an expression is non-nullable, as 
    /// in not `null` or `undefined`. Using assertions to tell the type system new information is often a sign that 
    /// code is not fully type-safe. It's generally better to structure program logic so that TypeScript understands 
    /// when values may be nullable.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface Example {
    ///   property?: string;
    /// }
    /// declare const example: Example;
    /// const includesBaz = foo.property!.includes('baz');
    /// ```
    /// 
    /// ### Valid
    /// 
    /// ```ts
    /// interface Example {
    ///   property?: string;
    /// }
    /// 
    /// declare const example: Example;
    /// const includesBaz = foo.property?.includes('baz') ?? false;
    /// ```
    ///
    pub(crate) NoNonNullAssertion {
        version: "11.0.0",
        name: "noNonNullAssertion",
        recommended: false,
    }
}

fn can_replace_with_optional_chain(node: &TsNonNullAssertionExpression) -> bool {
    match node.parent::<JsAnyExpression>() {
        Some(parent) => {
            matches!(
                parent.syntax().kind(),
                JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                    | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION
                    | JsSyntaxKind::JS_CALL_EXPRESSION
            )
        }
        None => false,
    }
}

impl Rule for NoNonNullAssertion {
    type Query = Ast<TsNonNullAssertionExpression>;
    type State = (TsNonNullAssertionExpression, bool);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if let Some(child) = node.syntax().first_child() {
            let is_child_non_null_assertion =
                matches!(child.kind(), JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION);
            if is_child_non_null_assertion {
                return None;
            }

            let most_outer_ts_non_null_assertion_node = node
                .syntax()
                .ancestors()
                .take_while(|n| -> bool {
                    n.kind() == JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
                })
                .last()?
                .cast::<TsNonNullAssertionExpression>()?;

            return Some((
                most_outer_ts_non_null_assertion_node.clone(),
                can_replace_with_optional_chain(&most_outer_ts_non_null_assertion_node),
            ));
        } else {
            return Some((node.clone(), can_replace_with_optional_chain(node)));
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (node, _) = state;
        Some(
    		RuleDiagnostic::new(
    			rule_category!(),
    			node.syntax().text_trimmed_range(),
    			markup! {
    				"Forbidden non-null assertion."
    			},
    		).note(
    			markup! {
    				"Consider using the optional chain operator "<Emphasis>"?."</Emphasis>" instead. This operator includes runtime checks, so it is safer than the compile-only non-null assertion operator.
                    "
    			},
    		)
    	)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let (node, can_fix) = state;
        if !can_fix {
            return None;
        }

        let mut mutation = ctx.root().begin();

        node.syntax()
            .descendants()
            .map_while(|x| {
                let child = x.first_child()?;
                if child.kind() == JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION {
                    Some(child.cast::<TsNonNullAssertionExpression>()?)
                } else {
                    None
                }
            })
            .for_each(|x| {
                if let Some(excl_token) = x.excl_token().ok() {
                    mutation.remove_token(excl_token);
                }
            });

        match node.parent::<JsAnyExpression>()? {
            JsAnyExpression::JsComputedMemberExpression(parent) => {
                if parent.is_optional() {
                    mutation.remove_token(node.excl_token().ok()?);
                } else {
                    mutation.replace_token(
                        node.excl_token().ok()?,
                        make::token(JsSyntaxKind::QUESTIONDOT),
                    );
                }
            }
            JsAnyExpression::JsCallExpression(parent) => {
                if parent.is_optional() {
                    mutation.remove_token(node.excl_token().ok()?);
                } else {
                    mutation.replace_token(
                        node.excl_token().ok()?,
                        make::token(JsSyntaxKind::QUESTIONDOT),
                    );
                }
            }
            JsAnyExpression::JsStaticMemberExpression(parent) => {
                if parent.is_optional() {
                    mutation.remove_token(node.excl_token().ok()?);
                } else {
                    mutation.replace_token(
                        node.excl_token().ok()?,
                        make::token(JsSyntaxKind::QUESTION),
                    );
                }
            }
            _ => {}
        };

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Replace with optional chain operator "<Emphasis>"?"</Emphasis>"." }
                .to_owned(),
            mutation,
        })
    }
}
