use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::{markup, MarkupBuf};
use rome_js_syntax::{JsxAttribute, JsxOpeningElement, JsxSelfClosingElement};
use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// Enforce that `onMouseOver`/`onMouseOut` are accompanied by `onFocus`/`onBlur` for keyboard-only users.
    /// It is important to take into account users with physical disabilities who cannot use a mouse,
    /// who use assistive technology or screenreader.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    ///    <div onMouseOver={() => {}} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    ///    <div onMouseOut={() => {}} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///     <div onMouseOver={() => {}} onFocus={() => {}} />
    ///     <div onMouseOut={() => {}} onBlur={() => {}} />
    ///     <div onMouseOver={() => {}} {...otherProps} />
    ///     <div onMouseOut={() => {}} {...otherProps} />
    ///     <div onMouseOver={() => {}} onFocus={() => {}} {...otherProps} />
    ///     <div onMouseOut={() => {}} onBlur={() => {}} {...otherProps} />
    /// </>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// [WCAG 2.1.1](https://www.w3.org/WAI/WCAG21/Understanding/keyboard)
    ///
    /// ## Resources
    ///
    /// - [WebAIM - JavaScript event handlers](https://webaim.org/techniques/javascript/eventhandlers)
    pub(crate) UseKeyWithMouseEvents {
        version: "10.0.0",
        name: "useKeyWithMouseEvents",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) JsxAnyElement = JsxOpeningElement | JsxSelfClosingElement
}

pub(crate) enum UseKeyWithMouseEventsState {
    MissingOnFocus,
    MissingOnBlur,
}

impl UseKeyWithMouseEventsState {
    fn message(&self) -> MarkupBuf {
        match self {
            UseKeyWithMouseEventsState::MissingOnBlur => {
                markup! {"onMouseOut must be accompanied by onBlur for accessibility."}.to_owned()
            }
            UseKeyWithMouseEventsState::MissingOnFocus => {
                markup! {"onMouseOver must be accompanied by onFocus for accessibility."}.to_owned()
            }
        }
    }
}

impl JsxAnyElement {
    fn is_custom_component(&self) -> Option<bool> {
        match self {
            JsxAnyElement::JsxSelfClosingElement(element) => {
                element.name().ok()?.as_jsx_name().map(|_| true)
            }
            JsxAnyElement::JsxOpeningElement(element) => {
                element.name().ok()?.as_jsx_name().map(|_| true)
            }
        }
    }

    fn has_spread_attribute(&self) -> bool {
        match self {
            JsxAnyElement::JsxSelfClosingElement(element) => element
                .attributes()
                .into_iter()
                .any(|attribute| attribute.as_jsx_spread_attribute().is_some()),
            JsxAnyElement::JsxOpeningElement(element) => element
                .attributes()
                .into_iter()
                .any(|attribute| attribute.as_jsx_spread_attribute().is_some()),
        }
    }

    fn find_on_mouse_over_attribute(&self) -> Option<JsxAttribute> {
        match self {
            JsxAnyElement::JsxSelfClosingElement(element) => {
                element.find_attribute_by_name("onMouseOver").ok()?
            }
            JsxAnyElement::JsxOpeningElement(element) => {
                element.find_attribute_by_name("onMouseOver").ok()?
            }
        }
    }

    fn find_on_mouse_out_attribute(&self) -> Option<JsxAttribute> {
        match self {
            JsxAnyElement::JsxSelfClosingElement(element) => {
                element.find_attribute_by_name("onMouseOut").ok()?
            }
            JsxAnyElement::JsxOpeningElement(element) => {
                element.find_attribute_by_name("onMouseOut").ok()?
            }
        }
    }

    fn find_on_focus_attribute(&self) -> Option<JsxAttribute> {
        match self {
            JsxAnyElement::JsxSelfClosingElement(element) => {
                element.find_attribute_by_name("onFocus").ok()?
            }
            JsxAnyElement::JsxOpeningElement(element) => {
                element.find_attribute_by_name("onFocus").ok()?
            }
        }
    }

    fn find_on_blur_attribute(&self) -> Option<JsxAttribute> {
        match self {
            JsxAnyElement::JsxSelfClosingElement(element) => {
                element.find_attribute_by_name("onBlur").ok()?
            }
            JsxAnyElement::JsxOpeningElement(element) => {
                element.find_attribute_by_name("onBlur").ok()?
            }
        }
    }

    fn has_focus_attributes(&self) -> Option<bool> {
        if self.find_on_mouse_over_attribute().is_some() {
            let on_focus_attribute = self.find_on_focus_attribute();

            if on_focus_attribute.is_none() || is_value_undefined_or_null(&on_focus_attribute?) {
                return None;
            }
            return Some(true);
        }
        Some(true)
    }

    fn has_blur_attributes(&self) -> Option<bool> {
        if self.find_on_mouse_out_attribute().is_some() {
            let on_blur_attribute = self.find_on_blur_attribute();

            if on_blur_attribute.is_none() || is_value_undefined_or_null(&on_blur_attribute?) {
                return None;
            }
            return Some(true);
        }
        Some(true)
    }
}

impl Rule for UseKeyWithMouseEvents {
    type Query = Semantic<JsxAnyElement>;
    type State = UseKeyWithMouseEventsState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.is_custom_component().is_none() || node.has_spread_attribute() {
            return None;
        }

        if node.has_focus_attributes().is_none() {
            return Some(UseKeyWithMouseEventsState::MissingOnFocus);
        }

        if node.has_blur_attributes().is_none() {
            return Some(UseKeyWithMouseEventsState::MissingOnBlur);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let footer_note_text = markup! {"Actions triggered using mouse events should have corresponding events to account for keyboard-only navigation."};

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                state.message(),
            )
            .footer_note(footer_note_text),
        )
    }
}

fn is_value_undefined_or_null(attribute: &JsxAttribute) -> bool {
    attribute
        .initializer()
        .and_then(|x| {
            let expression = x
                .value()
                .ok()?
                .as_jsx_expression_attribute_value()?
                .expression()
                .ok()?;

            if let Some(id) = expression.as_js_identifier_expression() {
                let name = id.name().ok()?.syntax().text_trimmed();

                return Some(name == "undefined");
            }

            expression
                .as_js_any_literal_expression()?
                .as_js_null_literal_expression()?;

            Some(true)
        })
        .unwrap_or(false)
}
