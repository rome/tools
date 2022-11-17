use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::*;
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow assignment operators in conditional expressions.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var x;
    /// if (x = 0) {
    ///     var b = 1;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function setHeight(someNode) {
    ///     "use strict";
    ///     do {
    ///         someNode.height = "100px";
    ///     } while (someNode = someNode.parentNode);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var x;
    /// if (x === 0) {
    ///     var b = 1;
    /// }
    /// ```
    ///
    /// ```js
    /// function setHeight(someNode) {
    ///     "use strict";
    ///     do {
    ///         someNode.height = "100px";
    ///     } while ((someNode = someNode.parentNode) !== null);
    /// }
    /// ```
    ///
    /// ```
    pub(crate) NoConditionalAssignment {
        version: "11.0.0",
        name: "noConditionalAssignment",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) ConditionalStatement = JsConditionalExpression | JsWhileStatement | JsDoWhileStatement | JsIfStatement | JsForStatement
}

impl Rule for NoConditionalAssignment {
    type Query = Ast<ConditionalStatement>;
    type State = JsAssignmentExpression;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let stmt = ctx.query();
        stmt.test()
            .map(|it| it.omit_parentheses())
            .and_then(into_js_assignment)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, expr: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            expr.syntax().text_trimmed_range(),
            markup! {
                "Expected a conditional expression and instead saw an assignment."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let op = state.operator().ok()?;
        if let JsAssignmentOperator::Assign = op {
            let mut mutation = ctx.root().begin();
            let token = state.operator_token().ok()?;
            mutation.replace_token(token, make::token(JsSyntaxKind::EQ3));
            Some(JsRuleAction {
                mutation,
                applicability: Applicability::MaybeIncorrect,
                category: ActionCategory::QuickFix,
                message: markup!("Did you mean '==='?").to_owned(),
            })
        } else {
            None
        }
    }
}

fn into_js_assignment(expr: JsAnyExpression) -> Option<JsAssignmentExpression> {
    if let JsAnyExpression::JsAssignmentExpression(e) = expr {
        Some(e)
    } else {
        None
    }
}

impl ConditionalStatement {
    fn test(&self) -> Option<JsAnyExpression> {
        match self {
            Self::JsConditionalExpression(it) => it.test().ok(),
            Self::JsWhileStatement(it) => it.test().ok(),
            Self::JsDoWhileStatement(it) => it.test().ok(),
            Self::JsIfStatement(it) => it.test().ok(),
            Self::JsForStatement(it) => it.test(),
        }
    }
}
