use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsFunctionDeclaration, JsFunctionExpression, JsLanguage, JsMethodClassMember, JsSyntaxKind,
};
use rome_rowan::{declare_node_union, AstNode, NodeOrToken, SyntaxNode, SyntaxToken};

declare_rule! {
    /// Require generator functions to contain `yield`.
    ///
    /// This rule generates warnings for generator functions that do not have the `yield` keyword.
    ///
    /// Source: [require-await](https://eslint.org/docs/latest/rules/require-yield).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function* foo() {
    ///   return 10;
    /// }
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// function* foo() {
    ///   yield 5;
    ///   return 10;
    /// }
    ///
    /// function foo() {
    ///   return 10;
    /// }
    ///
    /// // This rule does not warn on empty generator functions.
    /// function* foo() { }
    /// ```
    pub(crate) UseYield {
        version: "12.0.0",
        name: "useYield",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) AnyFunction = JsFunctionDeclaration | JsFunctionExpression | JsMethodClassMember
}

impl Rule for UseYield {
    type Query = Ast<AnyFunction>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let (start_token, function_body_syntax) = Some(match node {
            AnyFunction::JsFunctionDeclaration(func_declaration) => (
                func_declaration.star_token(),
                func_declaration.body().ok()?.statements().into_syntax(),
            ),
            AnyFunction::JsFunctionExpression(func_expression) => (
                func_expression.star_token(),
                func_expression.body().ok()?.statements().into_syntax(),
            ),
            AnyFunction::JsMethodClassMember(class_method) => (
                class_method.star_token(),
                class_method.body().ok()?.statements().into_syntax(),
            ),
        })?;

        if start_token.is_some() && !has_yield_kw(NodeOrToken::from(function_body_syntax))? {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {"This generator function does not have "<Emphasis>"yield"</Emphasis>"."},
        ))
    }
}

/// Traverses the syntax tree and verifies the presence of the yield keyword.
fn has_yield_kw(
    node: NodeOrToken<SyntaxNode<JsLanguage>, SyntaxToken<JsLanguage>>,
) -> Option<bool> {
    if node.kind() == JsSyntaxKind::YIELD_KW {
        return Some(true);
    }

    if node.kind() == JsSyntaxKind::FUNCTION_KW || node.as_token().is_some() {
        return Some(false);
    }

    for child in node.as_node()?.children_with_tokens() {
        if !has_yield_kw(child)? {
            continue;
        }

        return Some(true);
    }

    Some(false)
}
