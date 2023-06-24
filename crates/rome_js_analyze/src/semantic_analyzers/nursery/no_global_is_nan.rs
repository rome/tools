use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{AnyJsExpression, T};
use rome_rowan::{AstNode, BatchMutationExt};

declare_rule! {
    /// Use `Number.isNaN` instead of global `isNaN`.
    ///
    /// `Number.isNaN()` and `isNaN()` [have not the same behavior](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isNaN#description).
    /// When the argument to `isNaN()` is not a number, the value is first coerced to a number.
    /// `Number.isNaN()` does not perform this coercion.
    /// Therefore, it is a more reliable way to test whether a value is `NaN`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// isNaN({}); // true
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// Number.isNaN({}); // false
    /// ```
    ///
    pub(crate) NoGlobalIsNan {
        version: "next",
        name: "noGlobalIsNan",
        recommended: true,
    }
}

impl Rule for NoGlobalIsNan {
    type Query = Semantic<AnyJsExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        match node {
            AnyJsExpression::JsIdentifierExpression(expression) => {
                let name = expression.name().ok()?;
                if name.has_name("isNaN") && model.binding(&name).is_none() {
                    return Some(());
                }
            }
            AnyJsExpression::JsStaticMemberExpression(expression) => {
                let object_name = expression
                    .object()
                    .ok()?
                    .omit_parentheses()
                    .as_js_identifier_expression()?
                    .name()
                    .ok()?;
                let member = expression.member().ok()?;
                if object_name.is_global_this()
                    && model.binding(&object_name).is_none()
                    && member.as_js_name()?.value_token().ok()?.text_trimmed() == "isNaN"
                {
                    return Some(());
                }
            }
            AnyJsExpression::JsComputedMemberExpression(expression) => {
                let object_name = expression
                    .object()
                    .ok()?
                    .omit_parentheses()
                    .as_js_identifier_expression()?
                    .name()
                    .ok()?;
                let member = expression.member().ok()?.omit_parentheses();
                let member = member
                    .as_any_js_literal_expression()?
                    .as_js_string_literal_expression()?;
                if object_name.is_global_this()
                    && model.binding(&object_name).is_none()
                    && member.value_token().ok()?.text_trimmed() == "isNaN"
                {
                    return Some(());
                }
            }
            _ => (),
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    <Emphasis>"isNaN"</Emphasis>" is unsafe. It attempts a type coercion. Use "<Emphasis>"Number.isNaN"</Emphasis>" instead."
                },
            )
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isNaN#description">"the MDN documentation"</Hyperlink>" for more details."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let number_constructor =
            make::js_identifier_expression(make::js_reference_identifier(make::ident("Number")));
        let is_nan_name = make::js_name(make::ident("isNaN"));
        let expression = make::js_static_member_expression(
            number_constructor.into(),
            make::token(T![.]),
            is_nan_name.into(),
        );
        mutation.replace_node(node.clone(), expression.into());
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! {
                "Use "<Emphasis>"Number.isNaN"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        })
    }
}
