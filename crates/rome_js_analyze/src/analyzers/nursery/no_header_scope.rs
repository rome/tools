use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{
    JsxAnyElementName, JsxAttribute, JsxAttributeList, JsxOpeningElement, JsxSelfClosingElement,
};
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Check that the scope attribute is only used on `th` elements.
    ///
    /// ESLint Equivalent: [scope](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/master/docs/rules/scope.md)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div scope={scope} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div scope="col" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <th scope={scope}></th>
    /// ```
    ///
    /// ```jsx
    /// <th scope="col"></th>
    /// ```
    pub(crate) NoHeaderScope {
        version: "11.0.0",
        name: "noHeaderScope",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) JsxAnyElement = JsxOpeningElement | JsxSelfClosingElement
}

impl JsxAnyElement {
    fn name(&self) -> Option<JsxAnyElementName> {
        match self {
            JsxAnyElement::JsxOpeningElement(element) => element.name().ok(),
            JsxAnyElement::JsxSelfClosingElement(element) => element.name().ok(),
        }
    }
}

impl Rule for NoHeaderScope {
    type Query = Ast<JsxAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attr = ctx.query();

        if attr.name().ok()?.syntax().text_trimmed() != "scope" {
            return None;
        }

        let jsx_element = attr
            .parent::<JsxAttributeList>()?
            .parent::<JsxAnyElement>()?;

        if jsx_element.name()?.as_jsx_name()?.syntax().text_trimmed() != "th" {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {"Avoid using the "<Emphasis>"scope"</Emphasis>" attribute on elements other than "<Emphasis>"th"</Emphasis>" elements."}
                .to_owned(),
        );

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        mutation.remove_node(ctx.query().clone());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the "<Emphasis>"scope"</Emphasis>" attribute." }.to_owned(),
            mutation,
        })
    }
}
