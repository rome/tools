use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_js_syntax::{AnyJsxChild, JsxElement, TextRange};
use rome_rowan::AstNode;

declare_rule! {
    /// Enforces that `audio` and `video` elements must have a `track` for captions.
    ///
    /// **ESLint Equivalent:** [media-has-caption](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/media-has-caption.md)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    /// ```jsx,expect_diagnostic
    /// 	<video {...props} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// 	<audio>child</audio>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// 	<audio>
    /// 		<track kind="captions" {...props} />
    /// 	</audio>
    /// ```
    ///
    /// ```jsx
    /// 	<video muted {...props}></video>
    /// ```
    pub(crate) UseMediaCaption {
        version: "12.0.0",
        name: "useMediaCaption",
        recommended: true,
    }
}

impl Rule for UseMediaCaption {
    type Query = Ast<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let has_video = node.name_value_token()?.text_trimmed() == "video";
        let has_audio = node.name_value_token()?.text_trimmed() == "audio";
        let has_muted = node.find_attribute_by_name("muted").is_some();

        if !(has_video || has_audio) || has_muted {
            return None;
        }

        match node {
            AnyJsxElement::JsxOpeningElement(_) => {
                let jsx_element = node.parent::<JsxElement>()?;
                let has_track = jsx_element
                    .children()
                    .into_iter()
                    .filter_map(|child| {
                        let any_jsx = match child {
                            AnyJsxChild::JsxElement(element) => {
                                Some(AnyJsxElement::from(element.opening_element().ok()?))
                            }
                            AnyJsxChild::JsxSelfClosingElement(element) => {
                                Some(AnyJsxElement::from(element))
                            }
                            _ => None,
                        }?;

                        let has_track = any_jsx.name_value_token()?.text_trimmed() == "track";
                        let has_valid_kind = &any_jsx
                            .find_attribute_by_name("kind")?
                            .initializer()?
                            .value()
                            .ok()?
                            .as_jsx_string()?
                            .inner_string_text()
                            .ok()?
                            .to_lowercase()
                            == "captions";

                        Some(has_track && has_valid_kind)
                    })
                    .any(|is_valid| is_valid);

                if !has_track {
                    return Some(jsx_element.range());
                }
            }
            _ => return Some(node.range()),
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {"Provide a "<Emphasis>"track"</Emphasis>" for captions when using "<Emphasis>"audio"</Emphasis>" or "<Emphasis>"video"</Emphasis>" elements."}.to_owned(),
        )
        .note("Captions support users with hearing-impairments. They should be a transcription or translation of the dialogue, sound effects, musical cues, and other relevant audio information.");

        Some(diagnostic)
    }
}
