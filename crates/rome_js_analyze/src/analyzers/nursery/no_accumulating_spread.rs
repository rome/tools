use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsBindingPattern, AnyJsFormalParameter, AnyJsParameter, JsCallExpression, JsSpread,
};
use rome_rowan::{AstNode, AstSeparatedList};

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
    /// a.reduce((acc, val) => {...acc, [val]: val}, {});
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var a = ['a', 'b', 'c'];
    /// a.reduce((acc, val) => {acc.push(val); return acc}, []);
    /// ```
    ///
    /// @link: https://astexplorer.net/
    pub(crate) NoAccumulatingSpread {
        version: "next",
        name: "noAccumulatingSpread",
        recommended: false,
    }
}

impl Rule for NoAccumulatingSpread {
    type Query = Ast<JsSpread>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        is_known_accumulator(node)?.then_some(())
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

fn is_known_accumulator(node: &JsSpread) -> Option<bool> {
    let node_name = node.argument().ok()?.text();

    let call_expression = node.syntax().ancestors().find_map(JsCallExpression::cast)?;

    let expression_name = call_expression
        .callee()
        .ok()?
        .as_js_static_member_expression()?
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()?;
    let expression_name = expression_name.text_trimmed();
    if matches!(expression_name, "reduce" | "reduceRight") {
        // Get the callback function of the reduce call
        let first_arg = call_expression
            .arguments()
            .ok()?
            .args()
            .iter()
            .next()?
            .ok()?;

        // check if first_arg is a JsFunction
        let callback = first_arg
            .as_any_js_expression()?
            .as_js_arrow_function_expression()?;

        let param = callback
            .parameters()
            .ok()?
            .as_js_parameters()?
            .clone()
            .items()
            .iter()
            .next()?
            .ok()?;

        if let Some(binding) = binding_of(&param) {
            if let Some(binding) = binding.as_any_js_binding() {
                // TODO: figure out how to check if the spread operator (node) is part of the param binding
                if binding.text() == node_name {
                    return Some(true);
                }
            }
        }
        None
    } else {
        None
    }
}

fn binding_of(param: &AnyJsParameter) -> Option<AnyJsBindingPattern> {
    match param {
        AnyJsParameter::AnyJsFormalParameter(formal_param) => match &formal_param {
            AnyJsFormalParameter::JsBogusParameter(_) => None,
            AnyJsFormalParameter::JsFormalParameter(param) => param.binding().ok(),
        },
        AnyJsParameter::JsRestParameter(param) => param.binding().ok(),
        AnyJsParameter::TsThisParameter(_) => None,
    }
}
