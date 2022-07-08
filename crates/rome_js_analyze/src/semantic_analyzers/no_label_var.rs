use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsLabeledStatement, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::AstNode;

declare_rule! {
    ///  Disallow labels that share a name with a variable
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const x1 = "test";
    /// x1: expr;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const x = "test";
    /// z: expr;
    /// ```
    pub(crate) NoLabelVar = "noLabelVar"
}

impl Rule for NoLabelVar {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Semantic<JsLabeledStatement>;
    /// first element of tuple is `JsSyntaxNode` of binding, second element of tuple is `label_token`
    type State = (JsSyntaxNode, JsSyntaxToken);
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let label_statement = ctx.query();

        let label_token = label_statement.label_token().ok()?;
        let name = label_token.text_trimmed();
        let model = ctx.model();
        // We search each scope from current scope until the global scope
        // if we find a binding that has its name equal to label name, then we found a  `LabelVar` issue.
        for scope in model.scope(label_statement.syntax()).ancestors() {
            if let Some(binding) = scope.get_binding(name) {
                return Some((binding.syntax().clone(), label_token));
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (binding_syntax_node, label_token) = state;
        let name = label_token.text_trimmed();

        Some(RuleDiagnostic::warning(
            label_token.text_trimmed_range(),
            markup! {
                "Do not use the "{name}" variable name as a label"
            },
        )
        .secondary(binding_syntax_node.text_trimmed_range(), markup! {
            "The variable is declared here"
        },)
        .footer_note(markup! {"Creating a label with the same name as an in-scope variable leads to confusion."}))
    }

    fn action(_: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        None
    }
}
