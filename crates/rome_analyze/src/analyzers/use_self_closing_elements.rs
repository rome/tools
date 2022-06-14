use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyRoot, JsxAnyTag, JsxElement, JsxOpeningElementFields, TriviaPieceKind, T,
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
        if n.children().is_empty() {
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
		// check if previous `open_element` have a whitespace before `>`
        let need_extra_whitespace = if let Some(last_attribute) = attributes.last() {
            let trailing = last_attribute.syntax().last_trailing_trivia();
            if let Some(trailing) = trailing {
                !trailing.text().ends_with(" ")
            } else {
                true
            }
        } else {
            let name = name.clone().ok()?;
            let trailing = name.syntax().last_trailing_trivia();
            if let Some(trailing) = trailing {
                !trailing.text().ends_with(" ")
            } else {
                true
            }
        };
        let self_closing_element_builder = make::jsx_self_closing_element(
            l_angle_token.ok()?,
            name.ok()?,
            attributes,
            if need_extra_whitespace {
                make::token(T![/])
                    .with_leading_trivia(std::iter::once((TriviaPieceKind::Whitespace, " ")))
            } else {
                make::token(T![/])
            },
            r_angle_token.ok()?,
        );
        let self_closing_element = self_closing_element_builder.build();
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
