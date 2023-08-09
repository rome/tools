use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    global_identifier, AnyJsCallArgument, AnyJsExpression, JsInstanceofExpression, T,
};
use rome_rowan::{trim_leading_trivia_pieces, AstNode, BatchMutationExt};

declare_rule! {
    /// Use `Array.isArray()` instead of `instanceof Array`.
    ///
    /// In _JavaScript_ some array-like objects such as _arguments_ are not instances of the `Array` class.    ///
    /// Moreover, the global `Array` class can be different between two execution contexts.
    /// For instance, two frames in a web browser have a distinct `Array` class.
    /// Passing arrays across these contexts, results in arrays that are not instances of the contextual global `Array` class.
    /// To avoid these issues, use `Array.isArray()` instead of `instanceof Array`.
    /// See the [MDN docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/isArray) for more details.
    ///
    /// Source: https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/no-instanceof-array.md
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const xs = [];
    /// if (xs instanceof Array) {}
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// const xs = [];
    /// if (Array.isArray(xs)) {}
    /// ```
    ///
    pub(crate) UseIsArray {
        version: "next",
        name: "useIsArray",
        recommended: true,
    }
}

impl Rule for UseIsArray {
    type Query = Semantic<JsInstanceofExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let right = node.right().ok()?.omit_parentheses();
        let (reference, name) = global_identifier(&right)?;
        if name.text() != "Array" {
            return None;
        }
        model.binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Use "<Emphasis>"Array.isArray()"</Emphasis>" instead of "<Emphasis>"instanceof Array"</Emphasis>"."
                },
            )
            .note(markup! {
                <Emphasis>"instanceof Array"</Emphasis>" returns false for array-like objects and arrays from other execution contexts."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let array = node.right().ok()?;
        let array_trailing_trivia = array.syntax().last_trailing_trivia()?.pieces();
        let mut mutation = ctx.root().begin();
        let is_array = make::js_static_member_expression(
            array.with_trailing_trivia_pieces([])?,
            make::token(T![.]),
            make::js_name(make::ident("isArray")).into(),
        );
        let arg = AnyJsCallArgument::AnyJsExpression(node.left().ok()?.trim()?);
        let instanceof_trailing_trivia = node.instanceof_token().ok()?.trailing_trivia().pieces();
        let args = make::js_call_arguments(
            make::token(T!['(']).with_trailing_trivia_pieces(trim_leading_trivia_pieces(
                instanceof_trailing_trivia,
            )),
            make::js_call_argument_list([arg], []),
            make::token(T![')']).with_trailing_trivia_pieces(array_trailing_trivia),
        );
        let call = make::js_call_expression(is_array.into(), args).build();
        mutation.replace_node_discard_trivia(
            AnyJsExpression::JsInstanceofExpression(node.clone()),
            call.into(),
        );
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! {
                "Use "<Emphasis>"Array.isArray()"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        })
    }
}
