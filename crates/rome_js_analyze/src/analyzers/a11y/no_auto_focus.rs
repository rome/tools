use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{jsx_ext::AnyJsxElement, JsxAttribute};
use rome_rowan::{AstNode, BatchMutationExt};

declare_rule! {
    /// Enforce that autoFocus prop is not used on elements.
    ///
    /// Autofocusing elements can cause usability issues for sighted and non-sighted users, alike.
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
    ///
    /// ## Resources
    ///
    /// - [WHATWG HTML Standard, The autofocus attribute](https://html.spec.whatwg.org/multipage/interaction.html#attr-fe-autofocus)
    /// - [The accessibility of HTML 5 autofocus](https://brucelawson.co.uk/2009/the-accessibility-of-html-5-autofocus/)
    ///
    pub(crate) NoAutoFocus {
        version: "10.0.0",
        name: "noAutofocus",
        recommended: true,
    }
}

impl Rule for NoAutoFocus {
    type Query = Ast<AnyJsxElement>;
    type State = JsxAttribute;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_custom_component() {
            return None;
        }

        node.find_attribute_by_name("autoFocus")
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
                let element = attr.syntax().ancestors().find_map(AnyJsxElement::cast);
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
