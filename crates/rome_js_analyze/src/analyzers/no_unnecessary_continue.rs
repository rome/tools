use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyStatement, JsContinueStatement, JsForStatement, JsForStatementFields, JsLabeledStatement,
    JsSyntaxKind, JsSyntaxNode, T,
};
use rome_rowan::{AstNode, AstNodeExt};

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
    pub(crate) NoUnnecessaryContinue = "noUnnecessaryContinue"
}

impl Rule for NoUnnecessaryContinue {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsContinueStatement;
    type State = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
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

fn is_continue_un_necessary(node: &JsContinueStatement) -> Option<bool> {
    use rome_js_syntax::JsSyntaxKind::*;
    // node.syntax().ancestors()
    let mut syntax = node.into_syntax();
    let mut ancestors = vec![];
    let mut loop_stmt = None;
    while let Some(parent) = syntax.parent() {
        if !matches!(
            parent.kind(),
            JS_FOR_IN_STATEMENT | JS_FOR_OF_STATEMENT | JS_FOR_STATEMENT | JS_WHILE_STATEMENT
        ) {
            ancestors.push(parent);
        } else {
            loop_stmt = Some(parent);
            break;
        }
    }
    let loop_stmt = loop_stmt?;
    if ancestors.is_empty() {
        return Some(true);
    }
    todo!()
}

fn is_continue_last_statement(ancestors: &Vec<JsSyntaxNode>, syntax: JsSyntaxNode) -> Option<bool> {
    let first_node = ancestors.get(0)?;
    if first_node.kind() == JsSyntaxKind::JS_BLOCK_STATEMENT {
        Some(first_node.children().last()? == syntax)
    } else {
        None
    }
}

fn contains_parent_loop_label(node: JsSyntaxNode, loop_stmt: JsSyntaxNode) -> Option<bool> {
    let continue_stmt = JsContinueStatement::cast(node)?;
    let continue_stmt_label = continue_stmt.label_token();
	if let Some(label) = continue_stmt_label {
		let label_stmt = JsLabeledStatement::cast(loop_stmt.parent()?)?;
		Some(label_stmt.label_token().ok()?.text() == label.text())
	} else {
		Some(true)
	}
}


fn is_continue_inside_last_ancestors(ancestors: &Vec<JsSyntaxNode>, syntax: JsSyntaxNode) -> Option<bool> {
	let len = ancestors.len();
	for i in (1..len).rev() {
		let ancestor = ancestors[i];
		let child_len = ancestor.children().count();
	}
	todo!()
}
