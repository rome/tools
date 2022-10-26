use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{
    JsSyntaxToken, JsxAnyElementName, JsxAttribute, JsxOpeningElement, JsxSelfClosingElement,
};
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt};

declare_rule! {
    /// Avoid the `autoFocus` attribute
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus="true" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus={"false"} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus={undefined} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <input />
    ///```
    ///
    /// ```jsx
    /// <div />
    ///```
    ///
    /// ```jsx
    /// <button />
    ///```
    ///
    /// ```jsx
    /// // `autoFocus` prop in user created component is valid
    /// <MyComponent autoFocus={true} />
    ///```
    pub(crate) NoAutoFocus {
        version: "10.0.0",
        name: "noAutofocus",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) JsxAnyElement = JsxOpeningElement | JsxSelfClosingElement
}

impl JsxAnyElement {
    fn name_value_token(&self) -> Option<JsSyntaxToken> {
        let any_name = match self {
            JsxAnyElement::JsxOpeningElement(element) => element.name().ok()?,
            JsxAnyElement::JsxSelfClosingElement(element) => element.name().ok()?,
        };

        match any_name {
            JsxAnyElementName::JsxMemberName(member) => member.member().ok()?.value_token().ok(),
            JsxAnyElementName::JsxName(name) => name.value_token().ok(),
            JsxAnyElementName::JsxNamespaceName(name) => name.name().ok()?.value_token().ok(),
            JsxAnyElementName::JsxReferenceIdentifier(name) => name.value_token().ok(),
        }
    }
}

impl Rule for NoAutoFocus {
    type Query = Ast<JsxAnyElement>;
    type State = JsxAttribute;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            JsxAnyElement::JsxOpeningElement(element) => {
                element.name().ok()?.as_jsx_name()?;
                element.find_attribute_by_name("autoFocus").ok()?
            }
            JsxAnyElement::JsxSelfClosingElement(element) => {
                element.name().ok()?.as_jsx_name()?;
                element.find_attribute_by_name("autoFocus").ok()?
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, attr: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            attr.syntax().text_trimmed_range(),
            markup! {
                "Avoid the "<Emphasis>"autoFocus"</Emphasis>" attribute."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, attr: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let trailing_trivia = attr.syntax().last_trailing_trivia();
        if let Some(trailing_trivia) = trailing_trivia {
            if trailing_trivia.pieces().any(|piece| piece.is_comments()) {
                let element = attr.syntax().ancestors().find_map(JsxAnyElement::cast);
                if let Some(name) = element.and_then(|e| e.name_value_token()) {
                    let trivia_pieces = name
                        .trailing_trivia()
                        .pieces()
                        .chain(trailing_trivia.pieces())
                        .collect::<Vec<_>>();
                    let new_name = name.with_trailing_trivia_pieces(trivia_pieces);
                    mutation.replace_token_discard_trivia(name, new_name);
                }
            }
        }

        mutation.remove_node(attr.clone());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the "<Emphasis>"autoFocus"</Emphasis>" attribute." }
                .to_owned(),
            mutation,
        })
    }
}
