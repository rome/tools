use crate::semantic_services::Semantic;
use crate::shared::react::{is_react_create_element, PossibleCreateElement};
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::codespan::Severity;
use rome_console::markup;
use rome_js_syntax::{
    JsAnyExpression, JsCallExpression, JsLiteralMemberName, JsxAnyAttributeName, JsxAttribute,
};
use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// Prevent the usage of dangerous JSX props
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// function createMarkup() {
    ///     return { __html: 'child' }
    /// }
    /// <div dangerouslySetInnerHTML={createMarkup()}></div>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement('div', {
    ///     dangerouslySetInnerHTML: { __html: 'child' }
    /// });
    /// ```
    pub(crate) NoDangerouslySetInnerHtml {
        version: "0.10.0",
        name: "noDangerouslySetInnerHtml",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) JsAnyCreateElement = JsxAttribute | JsCallExpression
}

pub(crate) enum NoDangerState {
    Attribute(JsxAttribute),
    Property(JsLiteralMemberName),
}

impl Rule for NoDangerouslySetInnerHtml {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Semantic<JsAnyCreateElement>;
    type State = NoDangerState;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        match node {
            JsAnyCreateElement::JsxAttribute(jsx_attribute) => {
                let name = jsx_attribute.name().ok()?;
                match name {
                    JsxAnyAttributeName::JsxName(jsx_name) => {
                        if jsx_name.syntax().text_trimmed() == "dangerouslySetInnerHTML" {
                            return Some(NoDangerState::Attribute(jsx_attribute.clone()));
                        }
                    }
                    JsxAnyAttributeName::JsxNamespaceName(_) => return None,
                }
            }
            JsAnyCreateElement::JsCallExpression(call_expression) => {
                let callee = call_expression.callee().ok()?;
                let is_create_element = match callee {
                    JsAnyExpression::JsStaticMemberExpression(static_member) => {
                        is_react_create_element(PossibleCreateElement::from(static_member), model)
                    }
                    JsAnyExpression::JsIdentifierExpression(identifier_expression) => {
                        is_react_create_element(
                            PossibleCreateElement::from(identifier_expression),
                            model,
                        )
                    }
                    _ => return None,
                }?;

                if is_create_element {
                    // if we are inside a create element call, we inspect the second argument, which
                    // should be an object expression. We look for a member that has as name
                    // "dangerouslySetInnerHTML"
                    let arguments = call_expression.arguments().ok()?.args();
                    let mut arguments = arguments.into_iter();
                    if let (Some(_), Some(second_argument)) = (arguments.next(), arguments.next()) {
                        let second_argument = second_argument.ok()?;
                        let object_expression = second_argument
                            .as_js_any_expression()?
                            .as_js_object_expression()?;
                        let members = object_expression.members();
                        for member in members {
                            let member = member.ok()?;
                            let property_member =
                                member.as_js_property_object_member()?.name().ok()?;
                            let name = property_member.as_js_literal_member_name()?;

                            if name.syntax().text_trimmed() == "dangerouslySetInnerHTML" {
                                return Some(NoDangerState::Property(name.clone()));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let text_range = match state {
            NoDangerState::Attribute(jsx_attribute) => {
                let name = jsx_attribute.name().ok()?;
                name.syntax().text_trimmed_range()
            }
            NoDangerState::Property(property) => property.syntax().text_trimmed_range(),
        };

        let diagnostic = RuleDiagnostic::new(
            text_range,
            markup! {
                "Avoid passing content using the "<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
            }
                .to_owned(),
        ).footer(
            Severity::Warning,
            "Setting content using code can expose users to cross-site scripting (XSS) attacks",
        );
        Some(diagnostic)
    }
}
