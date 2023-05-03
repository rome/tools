use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::SemanticModel;
use rome_js_syntax::{
    AnyJsFunction, JsCallArgumentList, JsCallArguments, JsCallExpression, JsFormalParameter,
    JsParameterList, JsParameters, JsSpread,
};
use rome_rowan::AstNode;

use crate::semantic_services::Semantic;

declare_rule! {
    /// Disallow the use of spread (`...`) syntax on accumulators.
    ///
    /// Spread syntax allows an iterable to be expanded into its individual elements.
    ///
    /// Spread syntax should be avoided on accumulators (like those in `.reduce`)
    /// because it causes a time complexity of `O(n^2)` instead of `O(n)`.
    ///
    /// Source: https://prateeksurana.me/blog/why-using-object-spread-with-reduce-bad-idea/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = ['a', 'b', 'c'];
    /// a.reduce((acc, val) => [...acc, val], []);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = ['a', 'b', 'c'];
    /// a.reduce((acc, val) => {return [...acc, val];}, []);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = ['a', 'b', 'c'];
    /// a.reduce((acc, val) => ({...acc, [val]: val}), {});
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var a = ['a', 'b', 'c'];
    /// a.reduce((acc, val) => {acc.push(val); return acc}, []);
    /// ```
    ///
    pub(crate) NoAccumulatingSpread {
        version: "12.1.0",
        name: "noAccumulatingSpread",
        recommended: false,
    }
}

impl Rule for NoAccumulatingSpread {
    type Query = Semantic<JsSpread>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        is_known_accumulator(node, model)?.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Avoid the use of spread (`...`) syntax on accumulators."
                },
            )
            .note(markup! {
                "Spread syntax should be avoided on accumulators (like those in `.reduce`) because it causes a time complexity of `O(n^2)`."
            }),
        )
    }
}

fn is_known_accumulator(node: &JsSpread, model: &SemanticModel) -> Option<bool> {
    let reference = node
        .argument()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?;

    let parameter = model
        .binding(&reference)
        .and_then(|declaration| declaration.syntax().parent())
        .and_then(JsFormalParameter::cast)?;
    let function = parameter
        .parent::<JsParameterList>()
        .and_then(|list| list.parent::<JsParameters>())
        .and_then(|parameters| parameters.parent::<AnyJsFunction>())?;
    let call_expression = function
        .parent::<JsCallArgumentList>()
        .and_then(|arguments| arguments.parent::<JsCallArguments>())
        .and_then(|arguments| arguments.parent::<JsCallExpression>())?;

    let name = call_expression
        .callee()
        .ok()?
        .as_js_static_member_expression()?
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()?;
    let name = name.text_trimmed();

    if matches!(name, "reduce" | "reduceRight") {
        Some(parameter.syntax().index() == 0)
    } else {
        None
    }
}
