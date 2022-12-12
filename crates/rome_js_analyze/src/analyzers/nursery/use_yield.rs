use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsClass, AnyJsFunction, JsLanguage, JsMethodClassMember, JsMethodObjectMember,
    JsStatementList, JsSyntaxKind, WalkEvent,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeList, SyntaxNode};

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
    pub(crate) AnyFunctionLike = AnyJsFunction | JsMethodObjectMember | JsMethodClassMember
}

impl AnyFunctionLike {
    fn is_generator(&self) -> bool {
        match self {
            AnyFunctionLike::AnyJsFunction(any_js_function) => any_js_function.is_generator(),
            AnyFunctionLike::JsMethodClassMember(method_class_member) => {
                method_class_member.star_token().is_some()
            }
            AnyFunctionLike::JsMethodObjectMember(method_obj_member) => {
                method_obj_member.star_token().is_some()
            }
        }
    }

    fn statements(&self) -> Option<JsStatementList> {
        Some(match self {
            AnyFunctionLike::AnyJsFunction(any_js_function) => any_js_function
                .body()
                .ok()?
                .as_js_function_body()?
                .statements(),
            AnyFunctionLike::JsMethodClassMember(method_class_member) => {
                method_class_member.body().ok()?.statements()
            }
            AnyFunctionLike::JsMethodObjectMember(method_obj_member) => {
                method_obj_member.body().ok()?.statements()
            }
        })
    }
}

impl Rule for UseYield {
    type Query = Ast<AnyFunctionLike>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let function_body_statements = node.statements()?;

        if node.is_generator()
            && !function_body_statements.is_empty()
            && !has_yield_expression(function_body_statements.syntax())
        {
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

/// Traverses the syntax tree and verifies the presence of a yield expression.
fn has_yield_expression(node: &SyntaxNode<JsLanguage>) -> bool {
    let mut iter = node.preorder();

    while let Some(event) = iter.next() {
        match event {
            WalkEvent::Enter(enter) => {
                let kind = enter.kind();

                if kind == JsSyntaxKind::JS_YIELD_EXPRESSION {
                    return true;
                }

                if AnyJsClass::can_cast(kind) || AnyFunctionLike::can_cast(kind) {
                    iter.skip_subtree();
                }
            }
            WalkEvent::Leave(_) => {}
        };
    }

    return false;
}
