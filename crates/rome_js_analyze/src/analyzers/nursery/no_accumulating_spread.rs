use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsCallExpression, JsSpread};
use rome_rowan::AstNode;

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
    println!("----- is_known_accumulator -----");
    let call_expression = node.syntax().ancestors().find_map(JsCallExpression::cast)?;
    println!("call_expression: {:#?}", call_expression);

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
    println!("name: {:#?}", name);
    if matches!(name, "reduce" | "reduceRight") {
        // Check if node is the same variable as the first argument to the
        // function declaration.
        println!("node: {:#?}", node);
        println!("node.text: {:#?}", node.text());
        Some(true)
    } else {
        None
    }
}
