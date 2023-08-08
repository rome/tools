use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{global_identifier, AnyJsExpression, T};
use rome_rowan::{AstNode, BatchMutationExt};

declare_rule! {
    /// Use `Number.isFinite` instead of global `isFinite`.
    ///
    /// `Number.isFinite()` and `isFinite()` [have not the same behavior](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/isFinite#difference_between_number.isfinite_and_global_isfinite).
    /// When the argument to `isFinite()` is not a number, the value is first coerced to a number.
    /// `Number.isFinite()` does not perform this coercion.
    /// Therefore, it is a more reliable way to test whether a number is finite.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// isFinite(false); // true
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// Number.isFinite(false); // false
    /// ```
    pub(crate) NoGlobalIsFinite {
        version: "13.0.0",
        name: "noGlobalIsFinite",
        recommended: true,
    }
}

impl Rule for NoGlobalIsFinite {
    type Query = Semantic<AnyJsExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let (reference, name) = global_identifier(node)?;
        if name.text() != "isFinite" {
            return None;
        }
        model.binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    <Emphasis>"isFinite"</Emphasis>" is unsafe. It attempts a type coercion. Use "<Emphasis>"Number.isFinite"</Emphasis>" instead."
                },
            )
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isFinite#description">"the MDN documentation"</Hyperlink>" for more details."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let (old, new) = match node {
            AnyJsExpression::JsIdentifierExpression(expression) => (
                node.clone(),
                make::js_static_member_expression(
                    make::js_identifier_expression(make::js_reference_identifier(make::ident(
                        "Number",
                    )))
                    .into(),
                    make::token(T![.]),
                    make::js_name(expression.name().ok()?.value_token().ok()?).into(),
                ),
            ),
            AnyJsExpression::JsStaticMemberExpression(expression) => (
                node.clone(),
                make::js_static_member_expression(
                    make::js_static_member_expression(
                        expression.object().ok()?,
                        make::token(T![.]),
                        make::js_name(make::ident("Number")).into(),
                    )
                    .into(),
                    expression.operator_token().ok()?,
                    expression.member().ok()?,
                ),
            ),
            AnyJsExpression::JsComputedMemberExpression(expression) => {
                let object = expression.object().ok()?;
                (
                    object.clone(),
                    make::js_static_member_expression(
                        object,
                        make::token(T![.]),
                        make::js_name(make::ident("Number")).into(),
                    ),
                )
            }
            _ => return None,
        };
        mutation.replace_node(old, new.into());
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! {
                "Use "<Emphasis>"Number.isFinite"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        })
    }
}
