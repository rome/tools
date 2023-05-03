use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{AnyJsExpression, JsCallExpression, JsNewExpression, JsSyntaxToken};
use rome_rowan::{declare_node_union, AstNode, SyntaxResult};
use std::str::FromStr;

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
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        let callee = node.callee().ok()?.omit_parentheses();

        match callee {
            AnyJsExpression::JsIdentifierExpression(expression) => {
                let reference = expression.name().ok()?;
                let token = reference.value_token().ok()?;

                // verifies that the reference is not a local variable
                let is_global_call = is_non_callable_globals(token.text_trimmed())
                    && model.binding(&reference).is_none();
                is_global_call.then_some(reference.value_token().ok()?)
            }
            AnyJsExpression::JsStaticMemberExpression(expression) => {
                let object = expression.object().ok()?.omit_parentheses();
                let reference = object.as_reference_identifier()?;

                let member = expression.member().ok()?;
                let name = member.as_js_name()?;
                let token = name.value_token().ok()?;

                // verifies that the reference is not a local variable
                let is_global_call = is_non_callable_globals(token.text_trimmed())
                    && reference.is_global_this()
                    && model.binding(&reference).is_none();

                is_global_call.then_some(token)
            }
            AnyJsExpression::JsComputedMemberExpression(expression) => {
                let object = expression.object().ok()?.omit_parentheses();
                let reference = object.as_reference_identifier()?;
                let member = expression.member().ok()?.omit_parentheses();
                let literal = member
                    .as_any_js_literal_expression()?
                    .as_js_string_literal_expression()?;
                let name = literal.inner_string_text().ok()?;

                // verifies that the reference is not a local variable
                let is_global_call = is_non_callable_globals(name.text())
                    && reference.is_global_this()
                    && model.binding(&reference).is_none();

                is_global_call.then_some(literal.value_token().ok()?)
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, token: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let name = token.text_trimmed();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                <Emphasis>{name}</Emphasis>" is not a function."
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

#[derive(Debug, Eq, PartialEq)]
enum NonCallableGlobals {
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

fn is_non_callable_globals(text: &str) -> bool {
    NonCallableGlobals::from_str(text).is_ok()
}
