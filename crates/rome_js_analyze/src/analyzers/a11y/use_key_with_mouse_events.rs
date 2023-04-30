use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::{markup, MarkupBuf};
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_rowan::AstNode;

declare_rule! {
    /// Enforce `onMouseOver` / `onMouseOut` are accompanied by `onFocus` / `onBlur`.
    ///
    /// Coding for the keyboard is important for users with physical disabilities who cannot use a mouse, AT compatibility, and screenreader users.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div onMouseOver={() => {}} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div onMouseOut={() => {}} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///   <div onMouseOver={() => {}} onFocus={() => {}} />
    ///   <div onMouseOut={() => {}} onBlur={() => {}} />
    ///   <div onMouseOver={() => {}} {...otherProps} />
    ///   <div onMouseOut={() => {}} {...otherProps} />
    ///   <div onMouseOver={() => {}} onFocus={() => {}} {...otherProps} />
    ///   <div onMouseOut={() => {}} onBlur={() => {}} {...otherProps} />
    /// </>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.1.1](https://www.w3.org/WAI/WCAG21/Understanding/keyboard)
    ///
    pub(crate) UseKeyWithMouseEvents {
        version: "10.0.0",
        name: "useKeyWithMouseEvents",
        recommended: true,
    }
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

impl Rule for UseKeyWithMouseEvents {
    type Query = Semantic<AnyJsxElement>;
    type State = UseKeyWithMouseEventsState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if !node.is_custom_component() {
            if !has_valid_focus_attributes(node) {
                return Some(UseKeyWithMouseEventsState::MissingOnFocus);
            }

            if !has_valid_blur_attributes(node) {
                return Some(UseKeyWithMouseEventsState::MissingOnBlur);
            }
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
            .note(footer_note_text),
        )
    }
}

fn has_valid_focus_attributes(elem: &AnyJsxElement) -> bool {
    if let Some(on_mouse_over_attribute) = elem.find_attribute_by_name("onMouseOver") {
        if !elem.has_trailing_spread_prop(on_mouse_over_attribute) {
            return elem.find_attribute_by_name("onFocus").map_or(false, |it| {
                !it.as_static_value()
                    .map_or(false, |value| value.is_null_or_undefined())
            });
        }
    }
    true
}

fn has_valid_blur_attributes(elem: &AnyJsxElement) -> bool {
    if let Some(on_mouse_attribute) = elem.find_attribute_by_name("onMouseOut") {
        if !elem.has_trailing_spread_prop(on_mouse_attribute) {
            return elem.find_attribute_by_name("onBlur").map_or(false, |it| {
                !it.as_static_value()
                    .map_or(false, |value| value.is_null_or_undefined())
            });
        }
    }
    true
}
