use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make::{ident, js_call_expression, js_name, js_static_member_expression};
use rome_js_syntax::{JsAnyExpression, JsAnyName, JsCallExpression};
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
    /// let arr = ["1", "4", ["3", "6"]];
    /// arr.map(Number).flat();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let arr = ["1", "4", ["3", "6"]];
    /// arr.map(Number).flat(1);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let arr = ["1", "4", ["3", "6"]];
    /// arr.map(Number).flat(2);
    /// ```
    ///
    pub(crate) UseFlatMap {
        version: "10.0.0",
        name: "useFlatMap",
        recommended: false,
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
        if arguments.len() > 0 {
            let first_argument = arguments.iter().next();
            if let Some(first_argument) = first_argument {
                let first_argument = first_argument.ok()?;
                let first_argument = first_argument
                    .as_js_any_expression()?
                    .as_js_any_literal_expression()?
                    .as_js_number_literal_expression()?;

                if first_argument.value_token().ok()?.text_trimmed() != "1" {
                    return None;
                }
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
                "The call chain "<Emphasis>".flat().map()"</Emphasis>" can be replaced with a simple "<Emphasis>".flatMap()"</Emphasis>"."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let old_static_member_expression = state.callee().ok()?;
        let old_static_member_expression =
            old_static_member_expression.as_js_static_member_expression()?;
        let member = js_name(ident("flatMap"));
        let new_member_expression = js_static_member_expression(
            old_static_member_expression.object().ok()?,
            old_static_member_expression.operator_token().ok()?,
            JsAnyName::JsName(member),
        );
        let new_node = js_call_expression(
            JsAnyExpression::JsStaticMemberExpression(new_member_expression),
            state.arguments().ok()?,
        )
        .build();

        mutation.replace_node(node.clone(), new_node);

        Some(JsRuleAction {
            mutation,
            message: markup! {"Replace it with "<Emphasis>".flatMap()"</Emphasis>"."}.to_owned(),
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
        })
    }
}
