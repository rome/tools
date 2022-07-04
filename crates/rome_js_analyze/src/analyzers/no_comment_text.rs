use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsSyntaxKind, JsSyntaxToken, JsxText};
use rome_rowan::{AstNode, AstNodeExt};

use crate::JsRuleAction;

declare_rule! {
 	/// Prevent comments from being inserted as text nodes
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
	/// const a3 = <div>// comment</div>;
    /// ```
	///
	/// ```js,expect_diagnostic
	/// const a4 = <div>/* comment */</div>;
	/// ```
	///
	/// ```js,expect_diagnostic
	/// const a5 = <div>/** comment */</div>;
	/// ```
	///
	/// ### Valid
	///
	/// ```js
	/// const a = <div>{/* comment */}</div>;
	/// const a1 = <div>{/** comment */}</div>;
	/// const a2 = <div className={"cls" /* comment */}></div>;
	/// ```
    pub(crate) NoCommentText = "noCommentText"
}

// (\/\*\*|\/\*|\/\/)
impl Rule for NoCommentText {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsxText>;
    type State = String;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let Ast(n) = ctx.query();
        let jsx_value = n.text();
        let finalized_jsx_value = jsx_value
            .trim_start_matches("/**")
            .trim_start_matches("//")
            .trim_start_matches("/*")
            .trim_end_matches("*/")
            .to_string();
        if jsx_value == finalized_jsx_value {
            None
        } else {
            Some(finalized_jsx_value)
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let Ast(node) = ctx.query();

        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Wrap <Emphasis>comments</Emphasis> inside children within <Emphasis>braces</Emphasis>."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let Ast(node) = ctx.query();

        let root = ctx.root().replace_token(
            node.value_token().ok()?,
            JsSyntaxToken::new_detached(
                JsSyntaxKind::JSX_TEXT_LITERAL,
                &format!("{{/**{}*/}}", state),
                [],
                [],
            ),
        )?;

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Wrap the comments with braces" }.to_owned(),
            root,
        })
    }
}
