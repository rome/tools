use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::*;
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Enforces that no distracting elements are used.
    ///
    /// Elements that can be visually distracting can cause accessibility issues with visually impaired users.
    /// Such elements are most likely deprecated, and should be avoided.
    /// By default, the following elements are visually distracting: `<marquee>` and `<blink>`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <marquee/>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <blink/>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div/>
    /// ```
    pub(crate) NoDistractingElements {
        version: "11.0.0",
        name: "noDistractingElements",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) JsxAnyElement = JsxOpeningElement | JsxSelfClosingElement
}

impl JsxAnyElement {
    fn simple_jsx_name_token(&self) -> Option<JsSyntaxToken> {
        let name = match self {
            JsxAnyElement::JsxOpeningElement(element) => element.name().ok()?,
            JsxAnyElement::JsxSelfClosingElement(element) => element.name().ok()?,
        };
        match name {
            JsxAnyElementName::JsxName(name) => name.value_token().ok(),
            JsxAnyElementName::JsxReferenceIdentifier(ident) => ident.value_token().ok(),
            _ => None,
        }
    }
}

impl Rule for NoDistractingElements {
    type Query = Ast<JsxAnyElement>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let name = element.simple_jsx_name_token()?;
        match name.text_trimmed() {
            "marquee" | "blink" => Some(name),
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, name: &Self::State) -> Option<RuleDiagnostic> {
        let element = ctx.query();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            element.range(),
            markup! {"Don't use '"{name.text_trimmed()}"' element."}.to_owned(),
        )
        .note(markup! {
            "Visually distracting elements can cause accessibility issues and should be avoided"
        });

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, name: &Self::State) -> Option<JsRuleAction> {
        let element = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.remove_node(element.clone());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove '"{name.text_trimmed()}"' element." }.to_owned(),
            mutation,
        })
    }
}
