use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyRoot, JsxAnyTag, JsxElement, JsxOpeningElementFields, T,
};
use rome_rowan::{AstNode, AstNodeExt, AstNodeList};

use crate::{
    registry::{JsRuleAction, Rule, RuleDiagnostic},
    ActionCategory, RuleCategory,
};

pub(crate) enum UseSelfClosingElements {}

impl Rule for UseSelfClosingElements {
    const NAME: &'static str = "useSelfClosingElements";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsxElement;
    type State = ();

    fn run(n: &Self::Query) -> Option<Self::State> {
        if n.children().len() == 0 {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(node: &Self::Query, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "JSX elements without children should be marked as self-closing. In JSX, it is valid for any element to be self-closing."
            },
        ))
    }

    fn action(root: JsAnyRoot, node: &Self::Query, _: &Self::State) -> Option<JsRuleAction> {
        let open_element = node.opening_element().ok()?;
        let JsxOpeningElementFields {
            l_angle_token,
            name,
            type_arguments: _,
            attributes,
            r_angle_token,
        } = open_element.as_fields();
        let self_closing_element = make::jsx_self_closing_element(
            l_angle_token.ok()?,
            name.ok()?,
            attributes,
            make::token(T![/]),
            r_angle_token.ok()?,
        );
        let self_closing_element = self_closing_element.build();
        let root = root.replace_node(
            JsxAnyTag::JsxElement(node.clone()),
            JsxAnyTag::JsxSelfClosingElement(self_closing_element),
        )?;
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a SelfClosingElement instead" }.to_owned(),
            root,
        })
    }
}
