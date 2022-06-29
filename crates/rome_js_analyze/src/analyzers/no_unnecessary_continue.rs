use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{
    JsAnyRoot, JsContinueStatement, JsLabeledStatement, JsSyntaxKind, JsSyntaxNode,
};
use rome_rowan::{AstNode, SyntaxElement};

use crate::JsRuleAction;

declare_rule! {
    /// Avoid using unnecessary `ContinueStatement`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    /// ```js,expect_diagnostic
    /// loop: for (let i = 0; i < 5; i++) {
    ///   continue loop;
    /// }
    /// ```
    /// ```js,expect_diagnostic
    /// while (i--) {
    ///   continue;
    /// }
    /// ```
    /// ```js,expect_diagnostic
    /// while (1) {
    ///   continue;
    /// }
    /// ```
    /// ```js,expect_diagnostic
    /// for (let i = 0; i < 10; i++) {
    ///   if (i > 5) {
    ///     console.log("foo");
    ///     continue;
    ///   } else if (i >= 5 && i < 8) {
    ///     console.log("test");
    ///   } else {
    ///     console.log("test");
    ///   }
    /// }
    /// ```
    /// ```js,expect_diagnostic
    /// for (let i = 0; i < 9; i++) {
    ///   continue;
    /// }
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// while (i) {
    ///   if (i > 5) {
    ///     continue;
    ///   }
    ///   console.log(i);
    ///   i--;
    /// }
    ///
    /// loop: while (1) {
    ///   forLoop: for (let i = 0; i < 5; i++) {
    ///     if (someCondition) {
    ///       continue loop;
    ///     }
    ///   }
    /// }
    /// ```
    pub(crate) NoUnnecessaryContinue = "noUnnecessaryContinue"
}

impl Rule for NoUnnecessaryContinue {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsContinueStatement>;
    type State = ();
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let Ast(node) = ctx.query();
        is_continue_un_necessary(node)?.then(|| ())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let Ast(node) = ctx.query();
        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Unnecessary continue statement"
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let Ast(node) = ctx.query();
        let root_syntax = ctx.root().into_syntax();
        let syntax = node.clone().into_syntax();
        // Get parent of `ContinueStatement` SyntaxNode .
        let parent = syntax.parent()?;
        // Find index of `ContinueStatement` SyntaxNode in parent.
        let index = parent.children().position(|n| n == syntax)?;
        // Remove `ContinueStatement` SyntaxNode from parent.
        let next_parent = parent.clone().splice_slots(index..=index, []);
        let next_root_syntax = root_syntax.replace_child(
            SyntaxElement::Node(parent),
            SyntaxElement::Node(next_parent),
        )?;
        let root = JsAnyRoot::cast(next_root_syntax)?;
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Delete the unnecessary continue statement" }.to_owned(),
            root,
        })
    }
}

fn is_continue_un_necessary(node: &JsContinueStatement) -> Option<bool> {
    use rome_js_syntax::JsSyntaxKind::*;
    let syntax = node.clone().into_syntax();
    let mut current = syntax.clone();
    let mut ancestors = vec![];
    let mut loop_stmt = None;
    while let Some(parent) = current.parent() {
        if !matches!(
            parent.kind(),
            JS_FOR_IN_STATEMENT | JS_FOR_OF_STATEMENT | JS_FOR_STATEMENT | JS_WHILE_STATEMENT
        ) {
            ancestors.push(parent.clone());
        } else {
            loop_stmt = Some(parent);
            break;
        }
        current = parent;
    }
    let loop_stmt = loop_stmt?;
    if ancestors.is_empty() {
        return Some(true);
    }
    Some(
        is_continue_last_statement(&ancestors, syntax.clone()).unwrap_or(false)
            && contains_parent_loop_label(syntax.clone(), loop_stmt).unwrap_or(false)
            && is_continue_inside_last_ancestors(&ancestors, syntax).unwrap_or(false),
    )
}

fn is_continue_last_statement(ancestors: &[JsSyntaxNode], syntax: JsSyntaxNode) -> Option<bool> {
    let first_node = ancestors.get(0).unwrap();
    if first_node.kind() == JsSyntaxKind::JS_STATEMENT_LIST {
        Some(first_node.children().last().unwrap() == syntax)
    } else {
        None
    }
}

/// return true if continue label is undefined or equal to its parent's looplabel
fn contains_parent_loop_label(node: JsSyntaxNode, loop_stmt: JsSyntaxNode) -> Option<bool> {
    let continue_stmt = JsContinueStatement::cast(node)?;
    let continue_stmt_label = continue_stmt.label_token();
    if let Some(label) = continue_stmt_label {
        let label_stmt = JsLabeledStatement::cast(loop_stmt.parent()?)?;
        Some(label_stmt.label_token().ok()?.text_trimmed() == label.text_trimmed())
    } else {
        Some(true)
    }
}

fn is_continue_inside_last_ancestors(
    ancestors: &Vec<JsSyntaxNode>,
    syntax: JsSyntaxNode,
) -> Option<bool> {
    let len = ancestors.len();
    for i in (2..=len).rev() {
        let ancestor = &ancestors[i - 1];
        if ancestor.kind() == JsSyntaxKind::JS_STATEMENT_LIST
            && ancestor.children().next().is_some()
        {
            let body = ancestor.children();
            let last_body_node = body.last()?;
            if !((len == 1 && last_body_node == syntax)
                || (len > 1 && last_body_node == ancestors[i - 2]))
            {
                return Some(false);
            }
        }
    }
    Some(true)
}
