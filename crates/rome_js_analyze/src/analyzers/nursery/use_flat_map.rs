use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make::{ident, js_name, js_static_member_expression};
use rome_js_syntax::{JsAnyName, JsStaticMemberExpression};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

declare_rule! {
    /// Promotes the use of `.flatMap()` when `flat().map()` are used together.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let arr = ["1", "4", ["3", "6"]];
    /// arr.flat().map(Number);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let arr = ["1", "4", ["3", "6"]];
    /// arr.flat(1).map(Number);
    /// ```
    ///
    pub(crate) UseFlatMap {
        version: "10.0.0",
        name: "useFlatMap",
        recommended: false,
    }
}

impl Rule for UseFlatMap {
    type Query = Ast<JsStaticMemberExpression>;
    type State = JsStaticMemberExpression;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let flat_member_name = node.member().ok()?.as_js_name()?.value_token().ok()?;
        if flat_member_name.text_trimmed() == "map" {
            let call_expression = node.object().ok()?;
            let call_expression = call_expression.as_js_call_expression()?;
            // if we have `flat(1)`, we can't apply the rule
            if call_expression.arguments().ok()?.args().len() > 0 {
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

            if map_member_name.text_trimmed() == "flat" {
                return Some(map_static_member_expression.clone());
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

        let member = js_name(ident("flatMap"));
        let static_member_expression = js_static_member_expression(
            state.object().ok()?,
            state.operator_token().ok()?,
            JsAnyName::JsName(member),
        );

        mutation.replace_node(node.clone(), static_member_expression);

        Some(JsRuleAction {
            mutation,
            message: markup! {"Replace it with "<Emphasis>".flatMap()"</Emphasis>""}.to_owned(),
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
        })
    }
}
