use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::JsCallExpression;
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow the use of `console.log`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// console.log()
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// console.info("info");
    /// console.warn("warn");
    /// console.error("error");
    /// console.assert(true);
    /// console.table(["foo", "bar"]);
    /// const console = { log() {} };
    /// console.log();
    /// ```
    ///
    pub(crate) NoConsoleLog {
        version: "next",
        name: "noConsoleLog",
        recommended: false,
    }
}

impl Rule for NoConsoleLog {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expression = ctx.query();
        let model = ctx.model();
        let callee = call_expression.callee().ok()?;
        let callee = callee.as_js_static_member_expression()?;

        let member = callee.member().ok()?;
        let object = callee.object().ok()?;
        let object = object.as_js_identifier_expression()?;

        if member.as_js_name()?.value_token().ok()?.text_trimmed() == "log"
            && object.name().ok()?.value_token().ok()?.text_trimmed() == "console"
        {
            let binding = object.name().ok()?;
            let reference_binding = model.binding(&binding);
            if reference_binding.is_none() {
                return Some(());
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Don't use "<Emphasis>"console.log"</Emphasis>
                },
            )
            .note(markup! {
                <Emphasis>"console.log"</Emphasis>" is usually a tool for debugging and you don't want to have that in production."
            }),
        )
    }
}
