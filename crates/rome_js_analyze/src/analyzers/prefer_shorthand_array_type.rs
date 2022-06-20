use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{JsAnyStatement, JsForStatement, JsForStatementFields, T, TsReferenceType};
use rome_rowan::{AstNodeExt, AstNode};

use crate::JsRuleAction;

declare_rule! {
    /// Enforce the use of `while` loops instead of `for` loops when the
    /// initializer and update expressions are not needed
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (; x.running;) {
    ///     x.step();
    /// }
    /// ```
    pub(crate) PreferShorthandArrayType = "preferShorthandArrayType"
}

impl Rule for PreferShorthandArrayType {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = TsReferenceType;
    type State = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        if !is_array_reference(&node).unwrap_or(false) || node.type_arguments().is_none() {
            return None;
        }
        // SAFETY: We have checked the `node.type_arguments` is `Some` above, if it `None`, it would be early returned.
        let type_arguments = node.type_arguments().unwrap();

        todo!()
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        // SAFETY: These tokede.r_paren_token().unwrap().text_trimmed_range();

        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Use "<Emphasis>"while"</Emphasis>" loops instead of "<Emphasis>"for"</Emphasis>" loops."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let root = ctx.root();
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a while loop" }.to_owned(),
            root,
        })
    }
}

fn is_array_reference(ty: &TsReferenceType) -> Option<bool> {
    let name = ty.name().ok()?;
    name.as_js_reference_identifier().and_then(|identifier| {
        let name = identifier.value_token().ok()?;
        Some(name.text_trimmed() == "Array")
    })

}

fn convert_to_array_type(type_arguments: TsTypeArguments) Option<> {

}