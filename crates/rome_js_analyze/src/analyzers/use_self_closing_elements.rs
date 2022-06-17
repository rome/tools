use rome_analyze::{declare_rule, context::RuleContext, ActionCategory, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{ JsSyntaxToken, JsxAnyTag, JsxElement, JsxOpeningElementFields, T};
use rome_rowan::{AstNode, AstNodeExt, AstNodeList, TriviaPiece};

use crate::JsRuleAction;

declare_rule! {
    pub(crate) UseSelfClosingElements = "useSelfClosingElements"
}

impl Rule for UseSelfClosingElements {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsxElement;
    type State = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        if ctx.query().children().is_empty() {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::warning(
            ctx.query().range(),
            markup! {
                "JSX elements without children should be marked as self-closing. In JSX, it is valid for any element to be self-closing."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let open_element = ctx.query().opening_element().ok()?;
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
        // check if previous `open_element` have a whitespace before `>`
        // this step make sure we could convert <div></div> -> <div />
        // <div test="some""></div> -> <div test="some" />
        let prev_token = r_angle_token.prev_token();
        let need_extra_whitespace = prev_token
            .as_ref()
            .map_or(true, |token| !token.trailing_trivia().text().ends_with(' '));

        // drop the leading trivia of `r_angle_token`
        r_angle_token = r_angle_token.with_leading_trivia(std::iter::empty());

        if leading_trivia.is_empty() && need_extra_whitespace {
            slash_token.push(' ');
            leading_trivia.push(TriviaPiece::whitespace(1));
        }

        slash_token += "/";

        let mut self_closing_element_builder = make::jsx_self_closing_element(
            l_angle_token.ok()?,
            name.ok()?,
            attributes,
            JsSyntaxToken::new_detached(T![/], &slash_token, leading_trivia, []),
            r_angle_token,
        );
        if let Some(type_arguments) = type_arguments {
            self_closing_element_builder =
                self_closing_element_builder.with_type_arguments(type_arguments);
        }
        let self_closing_element = self_closing_element_builder.build();
        let root = ctx.root().replace_node(
            JsxAnyTag::JsxElement(ctx.query().clone()),
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
