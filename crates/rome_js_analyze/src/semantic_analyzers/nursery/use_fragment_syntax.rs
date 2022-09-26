use crate::react::{jsx_member_name_is_react_fragment, jsx_reference_identifier_is_fragment};
use crate::semantic_services::Semantic;
use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make::{
    jsx_child_list, jsx_closing_fragment, jsx_fragment, jsx_opening_fragment, token,
};
use rome_js_syntax::{JsxAnyElementName, JsxElement, T};
use rome_rowan::{AstNode, BatchMutationExt};

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
    const CATEGORY: RuleCategory = RuleCategory::Lint;
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

        if maybe_invalid
            && opening_element
                .find_attribute_by_name("key")
                .ok()?
                .is_none()
        {
            return Some(());
        }

        None
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let list = jsx_child_list(node.children());
        let fragment = jsx_fragment(
            jsx_opening_fragment(token(T![<]), token(T![>])),
            list,
            jsx_closing_fragment(token(T![<]), token(T![/]), token(T![>])),
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
