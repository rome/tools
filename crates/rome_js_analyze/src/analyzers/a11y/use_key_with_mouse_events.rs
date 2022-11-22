use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::{markup, MarkupBuf};
use rome_js_syntax::jsx_ext::JsxAnyElement;
use rome_rowan::AstNode;

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
    type Query = Semantic<JsxAnyElement>;
    type State = UseKeyWithMouseEventsState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.is_custom_component().is_some() {
            if node.has_valid_focus_attributes().is_none() {
                return Some(UseKeyWithMouseEventsState::MissingOnFocus);
            }

            if node.has_valid_blur_attributes().is_none() {
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
