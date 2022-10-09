use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
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
    /// ```jsx,expect_diagnostic
    ///     <div onMouseOver={() => {}} {...otherProps} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    ///     <div onMouseOut={() => {}} {...otherProps} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///     <div onMouseOver={() => {}} onFocus={() => {}} />
    ///     <div onMouseOut={() => {}} onBlur={() => {}} />
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

impl Rule for UseKeyWithMouseEvents {
    type Query = Semantic<JsxAnyElement>;
    type State = UseKeyWithMouseEventsState;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            JsxAnyElement::JsxSelfClosingElement(element) => {
                if element.name().ok()?.as_jsx_name().is_some() {
                    let on_mouse_over_attribute =
                        element.find_attribute_by_name("onMouseOver").ok()?;
                    if on_mouse_over_attribute.is_some() {
                        let on_focus_attribute = element.find_attribute_by_name("onFocus").ok()?;
                        if on_focus_attribute.is_none()
                            || is_value_undefined_or_null(&on_focus_attribute?)
                        {
                            return Some(UseKeyWithMouseEventsState::MissingOnFocus);
                        }
                    }

                    let on_mouse_out_attribute =
                        element.find_attribute_by_name("onMouseOut").ok()?;
                    if on_mouse_out_attribute.is_some() {
                        let on_blur_attribute = element.find_attribute_by_name("onBlur").ok()?;
                        if on_blur_attribute.is_none()
                            || is_value_undefined_or_null(&on_blur_attribute?)
                        {
                            return Some(UseKeyWithMouseEventsState::MissingOnBlur);
                        }
                    }
                }
            }
            JsxAnyElement::JsxOpeningElement(element) => {
                if element.name().ok()?.as_jsx_name().is_some() {
                    let on_mouse_over_attribute =
                        element.find_attribute_by_name("onMouseOver").ok()?;
                    if on_mouse_over_attribute.is_some() {
                        let on_focus_attribute = element.find_attribute_by_name("onFocus").ok()?;
                        if on_focus_attribute.is_none()
                            || is_value_undefined_or_null(&on_focus_attribute?)
                        {
                            return Some(UseKeyWithMouseEventsState::MissingOnFocus);
                        }
                    }

                    let on_mouse_out_attribute =
                        element.find_attribute_by_name("onMouseOut").ok()?;
                    if on_mouse_out_attribute.is_some() {
                        let on_blur_attribute = element.find_attribute_by_name("onBlur").ok()?;
                        if on_blur_attribute.is_none()
                            || is_value_undefined_or_null(&on_blur_attribute?)
                        {
                            return Some(UseKeyWithMouseEventsState::MissingOnBlur);
                        }
                    }
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let footer_note_text = markup! {"Actions triggered using mouse events should have corresponding events to account for keyboard-only navigation."};

        match state {
            UseKeyWithMouseEventsState::MissingOnBlur => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.syntax().text_trimmed_range(),
                    markup! {
                        "onMouseOut must be accompanied by onBlur for accessibility."
                    },
                )
                .footer_note(footer_note_text),
            ),
            UseKeyWithMouseEventsState::MissingOnFocus => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.syntax().text_trimmed_range(),
                    markup! {
                        "onMouseOver must be accompanied by onFocus for accessibility."
                    },
                )
                .footer_note(footer_note_text),
            ),
        }
    }
}

fn is_value_undefined_or_null(attribute: &JsxAttribute) -> bool {
    attribute
        .initializer()
        .and_then(|x| {
            let name = x
                .value()
                .ok()?
                .as_jsx_expression_attribute_value()?
                .expression()
                .ok()?
                .as_js_identifier_expression()?
                .name()
                .ok()?
                .syntax()
                .text_trimmed();
            Some(name == "undefined" || name == "null")
        })
        .unwrap_or(false)
}
