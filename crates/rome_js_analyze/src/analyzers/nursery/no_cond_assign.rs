use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::*;
use rome_rowan::{declare_node_union, AstNode};

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
    pub(crate) NoCondAssign {
        version: "11.0.0",
        name: "noCondAssign",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) ConditionalStatement = JsConditionalExpression | JsWhileStatement | JsDoWhileStatement | JsIfStatement | JsForStatement
}

impl Rule for NoCondAssign {
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
