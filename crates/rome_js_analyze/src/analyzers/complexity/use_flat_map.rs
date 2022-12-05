use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make::{ident, js_name};
use rome_js_syntax::{AnyJsExpression, AnyJsName, JsCallExpression};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

declare_rule! {
    /// Promotes the use of `.flatMap()` when `map().flat()` are used together.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const array = ["split", "the text", "into words"];
    /// array.map(sentence => sentence.split(' ')).flat();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const array = ["split", "the text", "into words"];
    /// array.map(sentence => sentence.split(' ')).flat(1);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const array = ["split", "the text", "into words"];
    /// array.map(sentence => sentence.split(' ')).flat(2);
    /// ```
    ///
    pub(crate) UseFlatMap {
        version: "10.0.0",
        name: "useFlatMap",
        recommended: true,
    }
}

impl Rule for UseFlatMap {
    type Query = Ast<JsCallExpression>;
    type State = JsCallExpression;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let arguments = node.arguments().ok()?.args();
        // Probably not a `flat` call.
        if arguments.len() > 1 {
            return None;
        }
        if let Some(first_argument) = arguments.first() {
            let first_argument = first_argument.ok()?;
            let first_argument = first_argument
                .as_any_js_expression()?
                .as_any_js_literal_expression()?
                .as_js_number_literal_expression()?;

            if first_argument.value_token().ok()?.text_trimmed() != "1" {
                return None;
            }
        }
        let static_member_expression = node.callee().ok()?;
        let static_member_expression = static_member_expression.as_js_static_member_expression()?;
        let flat_member_name = static_member_expression
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?;
        if flat_member_name.text_trimmed() == "flat" {
            let call_expression = static_member_expression.object().ok()?;
            let call_expression = call_expression.as_js_call_expression()?;
            let map_call_arguments = call_expression.arguments().ok()?.args();

            // probably not a `.map` all coming from an array
            if map_call_arguments.len() > 2 || map_call_arguments.len() < 1 {
                return None;
            }

            let map_static_member_expression = call_expression.callee().ok()?;
            let map_static_member_expression =
                map_static_member_expression.as_js_static_member_expression()?;
            let map_member_name = map_static_member_expression
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()?;

            if map_member_name.text_trimmed() == "map" {
                return Some(call_expression.clone());
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "The call chain "<Emphasis>".map().flat()"</Emphasis>" can be replaced with a single "<Emphasis>".flatMap()"</Emphasis>" call."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, flat_call: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let flat_call = flat_call.clone();
        let old_static_member_expression = flat_call.callee().ok()?;
        let old_static_member_expression =
            old_static_member_expression.as_js_static_member_expression()?;
        let member = js_name(ident("flatMap"));

        let flat_map_member_expression = old_static_member_expression
            .clone()
            .with_member(AnyJsName::JsName(member));

        let flat_map_call = flat_call.with_callee(AnyJsExpression::JsStaticMemberExpression(
            flat_map_member_expression,
        ));

        mutation.replace_node(node.clone(), flat_map_call);

        Some(JsRuleAction {
            mutation,
            message: markup! {"Replace the chain with "<Emphasis>".flatMap()"</Emphasis>"."}
                .to_owned(),
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
        })
    }
}
