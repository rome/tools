use crate::react::{jsx_member_name_is_react_fragment, jsx_reference_identifier_is_fragment};
use crate::semantic_services::Semantic;
use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make::{
    jsx_child_list, jsx_closing_fragment, jsx_fragment, jsx_opening_fragment,
};
use rome_js_syntax::{JsxAnyElementName, JsxElement};
use rome_rowan::{AstNode, AstNodeList, BatchMutationExt};

declare_rule! {
    /// This rule enforces the use of `<>...</>` over `<Fragment>...</Fragment>`.
    ///
    /// The shorthand fragment syntax saves keystrokes and is only inapplicable when keys are required.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <Fragment>child</Fragment>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <React.Fragment>child</React.Fragment>
    /// ```
    pub(crate) UseFragmentSyntax {
        version: "0.10.0",
        name: "useFragmentSyntax",
        recommended: false,
    }
}

impl Rule for UseFragmentSyntax {
    type Query = Semantic<JsxElement>;
    type State = ();
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        let opening_element = node.opening_element().ok()?;
        let name = opening_element.name().ok()?;
        let maybe_invalid = match name {
            JsxAnyElementName::JsxMemberName(member_name) => {
                jsx_member_name_is_react_fragment(&member_name, model)?
            }
            JsxAnyElementName::JsxReferenceIdentifier(identifier) => {
                jsx_reference_identifier_is_fragment(&identifier, model)?
            }
            JsxAnyElementName::JsxName(_) | JsxAnyElementName::JsxNamespaceName(_) => false,
        };

        if maybe_invalid && opening_element.attributes().is_empty() {
            return Some(());
        }

        None
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let list = jsx_child_list(node.children());
        let opening_element = node.opening_element().ok()?;
        let closing_element = node.closing_element().ok()?;
        let fragment = jsx_fragment(
            jsx_opening_fragment(
                opening_element.l_angle_token().ok()?,
                opening_element.r_angle_token().ok()?,
            ),
            list,
            jsx_closing_fragment(
                closing_element.l_angle_token().ok()?,
                closing_element.slash_token().ok()?,
                closing_element.r_angle_token().ok()?,
            ),
        );

        mutation.replace_element(
            node.clone().into_syntax().into(),
            fragment.into_syntax().into(),
        );

        Some(JsRuleAction {
            mutation,
            message:
                (markup! { "Replace "<Emphasis>"<Fragment>"</Emphasis>" with the fragment syntax" })
                    .to_owned(),
            applicability: Applicability::MaybeIncorrect,
            category: ActionCategory::QuickFix,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Use shorthand syntax for Fragment elements instead of standard syntax."
                },
            )
            .footer_note(markup! {
                "Shorthand fragment syntax saves keystrokes and is only inapplicable when keys are required."
            }),
        )
    }
}
