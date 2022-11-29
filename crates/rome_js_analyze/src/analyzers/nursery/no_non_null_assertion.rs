use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    AnyJsExpression, JsSyntaxKind, TsNonNullAssertionAssignment, TsNonNullAssertionExpression,
};
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt};

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
    /// ```ts,expect_diagnostic
    /// (b!! as number) = "test";
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

declare_node_union! {
    pub(crate) AnyTsNonNullAssertion = TsNonNullAssertionExpression | TsNonNullAssertionAssignment
}

impl Rule for NoNonNullAssertion {
    type Query = Ast<AnyTsNonNullAssertion>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyTsNonNullAssertion::TsNonNullAssertionExpression(node) => node
                .parent::<TsNonNullAssertionExpression>()
                .map_or(Some(()), |_| None),
            AnyTsNonNullAssertion::TsNonNullAssertionAssignment(node) => node
                .parent::<TsNonNullAssertionAssignment>()
                .map_or(Some(()), |_| None),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Forbidden non-null assertion."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        match node {
            AnyTsNonNullAssertion::TsNonNullAssertionAssignment(_) => None,
            AnyTsNonNullAssertion::TsNonNullAssertionExpression(node) => {
                if !can_replace_with_optional_chain(node) {
                    return None;
                }
                let mut mutation = ctx.root().begin();

                let assertions =
                    std::iter::successors(node.expression().ok(), |expression| match expression {
                        AnyJsExpression::TsNonNullAssertionExpression(assertion) => {
                            assertion.expression().ok()
                        }
                        _ => None,
                    });

                for assertion in assertions {
                    if let Some(non_null_expr) = assertion.as_ts_non_null_assertion_expression() {
                        mutation.remove_token(non_null_expr.excl_token().ok()?);
                    }
                }

                match node.parent::<AnyJsExpression>()? {
                    AnyJsExpression::JsComputedMemberExpression(parent) => {
                        if parent.is_optional() {
                            mutation.remove_token(node.excl_token().ok()?);
                        } else {
                            mutation.replace_token(
                                node.excl_token().ok()?,
                                make::token(JsSyntaxKind::QUESTIONDOT),
                            );
                        }
                    }
                    AnyJsExpression::JsCallExpression(parent) => {
                        if parent.is_optional() {
                            mutation.remove_token(node.excl_token().ok()?);
                        } else {
                            mutation.replace_token(
                                node.excl_token().ok()?,
                                make::token(JsSyntaxKind::QUESTIONDOT),
                            );
                        }
                    }
                    AnyJsExpression::JsStaticMemberExpression(parent) => {
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
                    message: markup! { "Replace with optional chain operator "<Emphasis>"?."</Emphasis>" This operator includes runtime checks, so it is safer than the compile-only non-null assertion operator" }
                        .to_owned(),
                    mutation,
                })
            }
        }
    }
}

fn can_replace_with_optional_chain(node: &TsNonNullAssertionExpression) -> bool {
    match node.parent::<AnyJsExpression>() {
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
