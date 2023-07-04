use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{global_identifier, AnyJsExpression, JsCallExpression, JsNewExpression};
use rome_rowan::{declare_node_union, SyntaxResult, TextRange};
use std::{fmt::Display, str::FromStr};

declare_rule! {
    /// Disallow calling global object properties as functions
    ///
    /// ECMAScript provides several global objects that are intended to be used as-is.
    /// Some of these objects look as if they could be constructors due their capitalization (such as Math and JSON) but will throw an error if you try to execute them as functions.
    ///
    /// The ECMAScript 5 specification makes it clear that both Math and JSON cannot be invoked:
    /// The Math object does not have a [[Call]] internal property; it is not possible to invoke the Math object as a function.
    ///
    /// The ECMAScript 2015 specification makes it clear that Reflect cannot be invoked:
    /// The Reflect object also does not have a [[Call]] internal method; it is not possible to invoke the Reflect object as a function.
    ///
    /// The ECMAScript 2017 specification makes it clear that Atomics cannot be invoked:
    /// The Atomics object does not have a [[Call]] internal method; it is not possible to invoke the Atomics object as a function.
    ///
    /// And the ECMAScript Internationalization API Specification makes it clear that Intl cannot be invoked:
    /// The Intl object does not have a [[Call]] internal method; it is not possible to invoke the Intl object as a function.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var math = Math();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var newMath = new Math();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var json = JSON();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var newJSON = new JSON();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var reflect = Reflect();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var newReflect = new Reflect();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var atomics = Atomics();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var newAtomics = new Atomics();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var intl = Intl();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var newIntl = new Intl();
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// function area(r) {
    ///     return Math.PI * r * r;
    /// }
    ///
    /// var object = JSON.parse("{}");
    ///
    /// var value = Reflect.get({ x: 1, y: 2 }, "x");
    ///
    /// var first = Atomics.load(foo, 0);
    ///
    /// var segmenterFr = new Intl.Segmenter("fr", { granularity: "word" });
    /// ```
    ///
    pub(crate) NoGlobalObjectCalls {
        version: "12.0.0",
        name: "noGlobalObjectCalls",
        recommended: true,
    }
}

impl Rule for NoGlobalObjectCalls {
    type Query = Semantic<QueryNode>;
    type State = (NonCallableGlobals, TextRange);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let callee = node.callee().ok()?.omit_parentheses();
        let (reference, name) = global_identifier(&callee)?;
        let non_callable = NonCallableGlobals::from_str(name.text()).ok()?;
        model
            .binding(&reference)
            .is_none()
            .then_some((non_callable, name.token().text_trimmed_range()))
    }

    fn diagnostic(
        _: &RuleContext<Self>,
        (non_callable, range): &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                <Emphasis>{non_callable.to_string()}</Emphasis>" is not a function."
            },
        ))
    }
}

declare_node_union! {
    /// Enum for [JsCallExpression] and [JsNewExpression]
    pub(crate) QueryNode  = JsNewExpression  | JsCallExpression
}

impl QueryNode {
    fn callee(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            QueryNode::JsNewExpression(expression) => expression.callee(),
            QueryNode::JsCallExpression(expression) => expression.callee(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum NonCallableGlobals {
    Atomics,
    Json,
    Math,
    Reflect,
    Intl,
}

impl FromStr for NonCallableGlobals {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Atomics" => Ok(NonCallableGlobals::Atomics),
            "JSON" => Ok(NonCallableGlobals::Json),
            "Math" => Ok(NonCallableGlobals::Math),
            "Reflect" => Ok(NonCallableGlobals::Reflect),
            "Intl" => Ok(NonCallableGlobals::Intl),
            _ => Err("non callable globals not implemented".to_string()),
        }
    }
}

impl Display for NonCallableGlobals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            NonCallableGlobals::Atomics => "Atomics",
            NonCallableGlobals::Json => "Json",
            NonCallableGlobals::Math => "Math",
            NonCallableGlobals::Reflect => "Reflect",
            NonCallableGlobals::Intl => "Intl",
        };
        write!(f, "{}", repr)
    }
}
