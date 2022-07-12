use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyStatement, JsForStatement, JsForStatementFields, JsIfStatement,
    JsSyntaxNode, T,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeExt, SyntaxNodeCast};

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
    pub(crate) NoExtraBooleanCast = "noExtraBooleanCast"
}

fn is_in_boolean_context(node: JsSyntaxNode, parent: JsSyntaxNode) -> Option<bool> {
    parent
        .cast::<JsIfStatement>()
        .and_then(|stmt| Some(stmt.test().ok()?.syntax() == &node))
		.unwrap_or_else(||)
}

impl Rule for NoExtraBooleanCast {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsAnyExpression>;
    type State = ();
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Use "<Emphasis>"while"</Emphasis>" loops instead of "<Emphasis>"for"</Emphasis>" loops."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let root = ctx.root();
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a while loop" }.to_owned(),
            root,
        })
    }
}
