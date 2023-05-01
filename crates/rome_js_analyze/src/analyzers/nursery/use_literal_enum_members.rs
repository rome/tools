use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsExpression, JsBinaryExpression, JsSyntaxKind, JsUnaryExpression, JsUnaryOperator,
    TsEnumMember,
};
use rome_rowan::AstNode;

declare_rule! {
    /// Require all enum members to be literal values.
    ///
    /// Usually, an enum member is initialized with a literal number or a literal string.
    /// However, _TypeScript_ allows the value of an enum member to be many different kinds of expressions.
    /// Using a computed enum member is often error-prone and confusing.
    /// This rule requires the initialization of enum members with literal values.
    /// It allows bitwise expressions for supporting [enum flags](https://stackoverflow.com/questions/39359740/what-are-enum-flags-in-typescript/39359953#39359953).
    ///
    /// In contrast to the equivalent _ESLint_ rule, this rule allows arbitrary bitwise constant expressions.
    ///
    /// Source: https://typescript-eslint.io/rules/prefer-literal-enum-member/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// const x = 2;
    /// enum Computed {
    ///     A,
    ///     B = x,
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// const x = 2;
    /// enum Invalid {
    ///     A,
    ///     B = 2**3,
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```ts
    /// enum Direction {
    ///     Left,
    ///     Right,
    /// }
    /// ```
    ///
    /// ```ts
    /// enum Order {
    ///     Less = -1,
    ///     Equal = 0,
    ///     Greater = 1,
    /// }
    /// ```
    ///
    /// ```ts
    /// enum State {
    ///     Open = "Open",
    ///     Close = "Close",
    /// }
    /// ```
    ///
    /// ```ts
    /// enum FileAccess {
    ///     None = 0,
    ///     Read = 1,
    ///     Write = 1 << 1,
    ///     All = 1 | (1 << 1)
    /// }
    /// ```
    pub(crate) UseLiteralEnumMembers {
        version: "12.1.0",
        name: "useLiteralEnumMembers",
        recommended: true,
    }
}

impl Rule for UseLiteralEnumMembers {
    type Query = Ast<TsEnumMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let enum_member = ctx.query();
        let Some(initializer) = enum_member.initializer() else {
            // no initializer => sequentially assigned literal integer
            return None;
        };
        let expr = initializer.expression().ok()?.omit_parentheses();
        if expr.as_any_js_literal_expression().is_some() || is_bitwise_constant_expression(&expr) {
            return None;
        } else if let Some(expr) = expr.as_js_unary_expression() {
            if expr.is_signed_numeric_literal().ok()? {
                return None;
            }
        } else if let Some(expr) = expr.as_js_template_expression() {
            if expr.is_constant() {
                return None;
            }
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let enum_member = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            enum_member.initializer()?.expression().ok()?.range(),
            markup! {
                "The enum member should be initialized with a literal value such as a number or a string."
            },
        ))
    }
}

/// Returns true if `expr` is an expression that only includes literal numbers and bitwise operations.
fn is_bitwise_constant_expression(expr: &AnyJsExpression) -> bool {
    for node in expr.syntax().descendants() {
        if let Some(exp) = JsUnaryExpression::cast_ref(&node) {
            if exp.operator() != Ok(JsUnaryOperator::BitwiseNot) {
                return false;
            }
        } else if let Some(exp) = JsBinaryExpression::cast_ref(&node) {
            if !exp.is_binary_operator() {
                return false;
            }
        } else if !matches!(
            node.kind(),
            JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION | JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
        ) {
            return false;
        }
    }
    true
}
