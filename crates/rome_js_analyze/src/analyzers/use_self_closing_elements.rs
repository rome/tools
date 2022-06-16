use rome_analyze::{ActionCategory, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{JsAnyRoot, JsSyntaxToken, JsxAnyTag, JsxElement, JsxOpeningElementFields, T};
use rome_rowan::{AstNode, AstNodeExt, AstNodeList, TriviaPiece};

use crate::JsRuleAction;

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
            type_arguments,
            attributes,
            r_angle_token,
        } = open_element.as_fields();
        let mut r_angle_token = r_angle_token.ok()?;
        let mut leading_trivia = vec![];
        let mut slash_token = String::new();

        for trivia in r_angle_token.leading_trivia().pieces() {
            leading_trivia.push(TriviaPiece::new(trivia.kind(), trivia.text_len()));
            slash_token.push_str(trivia.text());
        }
        r_angle_token = r_angle_token.with_leading_trivia(std::iter::empty());
        // check if previous `open_element` have a whitespace before `>`
        // this step make sure we could convert <div></div> -> <div />
        // <div test="some""></div> -> <div test="some" />
        let need_extra_whitespace = if let Some(last_attribute) = attributes.last() {
            let trailing = last_attribute.syntax().last_trailing_trivia();
            if let Some(trailing) = trailing {
                !trailing.text().ends_with(' ')
            } else {
                true
            }
        } else {
            let name = name.clone().ok()?;
            let trailing = name.syntax().last_trailing_trivia();
            if let Some(trailing) = trailing {
                !trailing.text().ends_with(' ')
            } else {
                true
            }
        };

        if leading_trivia.is_empty() && need_extra_whitespace {
            slash_token.push(' ');
            leading_trivia.push(TriviaPiece::whitespace(1));
        }

        slash_token += "/";

        let self_closing_element_builder = make::jsx_self_closing_element(
            l_angle_token.ok()?,
            name.ok()?,
            attributes,
            JsSyntaxToken::new_detached(T![/], &slash_token, leading_trivia, []),
            r_angle_token,
            type_arguments,
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
