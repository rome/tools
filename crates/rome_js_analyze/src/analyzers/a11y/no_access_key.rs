use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{
    AnyJsxAttributeValue, JsxAttribute, JsxAttributeList, JsxOpeningElement, JsxSelfClosingElement,
};
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt};

declare_rule! {
    /// Enforce that the `accessKey` attribute is not used on any HTML element.
    ///
    /// The `accessKey` assigns a keyboard shortcut to the current element. However, the `accessKey` value
    /// can conflict with keyboard commands used by screen readers and keyboard-only users, which leads to
    /// inconsistent keyboard actions across applications. To avoid accessibility complications,
    /// this rule suggests users remove the `accessKey` attribute on elements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <input type="submit" accessKey="s" value="Submit" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a href="https://webaim.org/" accessKey="w">WebAIM.org</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <button accessKey="n">Next</button>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [WebAIM: Keyboard Accessibility - Accesskey](https://webaim.org/techniques/keyboard/accesskey#spec)
    /// - [MDN `accesskey` documentation](https://developer.mozilla.org/docs/Web/HTML/Global_attributes/accesskey)
    ///
    pub(crate) NoAccessKey {
        version: "11.0.0",
        name: "noAccessKey",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) AnyJsxElement = JsxOpeningElement | JsxSelfClosingElement
}

impl AnyJsxElement {
    /// Check if the given element is a HTML element, not a user-created React component
    fn is_html_element(&self) -> Option<bool> {
        Some(match self {
            AnyJsxElement::JsxOpeningElement(element) => {
                element.name().ok()?.as_jsx_name().is_some()
            }
            AnyJsxElement::JsxSelfClosingElement(element) => {
                element.name().ok()?.as_jsx_name().is_some()
            }
        })
    }
}

impl Rule for NoAccessKey {
    type Query = Ast<JsxAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let attribute_name = node.name().ok()?;
        let name = attribute_name.as_jsx_name()?;
        let name_token = name.value_token().ok()?;
        if name_token.text_trimmed() != "accessKey" {
            return None;
        }

        let element = node
            .parent::<JsxAttributeList>()
            .and_then(|list| list.parent::<AnyJsxElement>())?;

        // We do not know if the `accessKey` prop is used for HTML elements
        // or for user-created React components
        if !element.is_html_element()? {
            return None;
        }

        let attribute_value = node.initializer()?.value().ok();
        if let Some(AnyJsxAttributeValue::JsxExpressionAttributeValue(expression)) = attribute_value
        {
            let expression = expression.expression().ok()?;
            let name = expression.as_js_identifier_expression()?.name().ok()?;
            let name_token = name.value_token().ok()?;
            if name_token.text_trimmed() == "undefined" {
                return None;
            }
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
    		RuleDiagnostic::new(
    			rule_category!(),
    			node.syntax().text_trimmed_range(),
    			markup! {
    				"Avoid the "<Emphasis>"accessKey"</Emphasis>" attribute to reduce inconsistencies between \
    				keyboard shortcuts and screen reader keyboard comments."
    			},
    		).note(
    			markup! {
    				"Assigning keyboard shortcuts using the "<Emphasis>"accessKey"</Emphasis>" attribute leads to \
    				inconsistent keyboard actions across applications."
    			},
    		)
    	)
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.remove_node(node.clone());
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the "<Emphasis>"accessKey"</Emphasis>" attribute." }
                .to_owned(),
            mutation,
        })
    }
}
