use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{jsx_ext::AnyJsxElement, JsxAttribute, JsxChildList, JsxElement};
use rome_rowan::{AstNode, AstNodeList};

declare_rule! {
    /// Enforces the usage of the `title` element for the `svg` element.
    ///
    /// It is not possible to specify the `alt` attribute for the `svg` as for the `img`.
    /// To make svg accessible, the following methods are available:
    /// - provide the `title` element as the first child to `svg`
    /// - provide `role="img"` and `aria-label` or `aria-labelledby` to `svg`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <svg>foo</svg>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <svg>
    ///     <title></title>
    ///     <circle />
    /// </svg>
    /// ``
    ///
    /// ```js,expect_diagnostic
    /// <svg>foo</svg>
    /// ```
    ///
    /// ```js
    /// <svg role="img" aria-label="">
    ///     <span id="">Pass</span>
    /// </svg>
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// <svg>
    ///     <rect />
    ///     <rect />
    ///     <g>
    ///         <circle />
    ///         <circle />
    ///         <g>
    ///             <title>Pass</title>
    ///             <circle />
    ///             <circle />
    ///         </g>
    ///     </g>
    /// </svg>
    /// ```
    ///
    /// ```js
    /// <svg>
    ///     <title>Pass</title>
    ///     <circle />
    /// </svg>
    /// ```
    ///
    /// ```js
    /// <svg role="img" aria-label="title">
    ///     <span id="title">Pass</span>
    /// </svg>
    /// ```
    ///
    /// ## Accessibility guidelines
    /// [Document Structure – SVG 1.1 (Second Edition)](https://www.w3.org/TR/SVG11/struct.html#DescriptionAndTitleElements)
    /// [ARIA: img role - Accessibility | MDN](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/img_role)
    /// [Accessible SVGs | CSS-Tricks - CSS-Tricks](https://css-tricks.com/accessible-svgs/)
    /// [Contextually Marking up accessible images and SVGs | scottohara.me](https://www.scottohara.me/blog/2019/05/22/contextual-images-svgs-and-a11y.html)
    ///
    pub(crate) NoSvgWithoutTitle {
        version: "12.0.0",
        name: "noSvgWithoutTitle",
        recommended: true,
    }
}

impl Rule for NoSvgWithoutTitle {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.name_value_token()?.text_trimmed() != "svg" {
            return None;
        }

        // Checks if a `svg` element has a valid `title` element is in a childlist
        let jsx_element = node.parent::<JsxElement>()?;
        if let AnyJsxElement::JsxOpeningElement(_) = node {
            let has_valid_title = has_valid_title_element(&jsx_element.children());
            if has_valid_title.map_or(false, |bool| bool) {
                return None;
            }
        }

        // Checks if a `svg` element has role='img' and title/aria-label/aria-labelledby attrigbute
        let Some(role_attribute) = node.find_attribute_by_name("role") else {
            return Some(())
        };

        let role_attribute_value = role_attribute.initializer()?.value().ok()?;
        let Some(text) = role_attribute_value.as_jsx_string()?.inner_string_text().ok() else {
            return Some(())
        };

        if text.to_lowercase() == "img" {
            let [aria_label, aria_labelledby] = node
                .attributes()
                .find_by_names(["aria-label", "aria-labelledby"]);

            let jsx_child_list = jsx_element.children();
            let is_valid = is_valid_attribute_value(aria_label, &jsx_child_list).unwrap_or(false)
                || is_valid_attribute_value(aria_labelledby, &jsx_child_list).unwrap_or(false);

            if !is_valid {
                return Some(());
            }
        };

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let diagnostic = RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Alternative text "<Emphasis>"title"</Emphasis>" element cannot be empty"
                },
            )
            .note(markup! {
                "For accessibility purposes, "<Emphasis>"SVGs"</Emphasis>" should have an alternative text,
            provided via "<Emphasis>"title"</Emphasis>" element. If the svg element has role=\"img\", you should add the "<Emphasis>"aria-label"</Emphasis>" or "<Emphasis>"aria-labelledby"</Emphasis>" attribute."
            });
        Some(diagnostic)
    }
}

/// Checks if the given attribute is attached to the `svg` element and the attribute value is used by the `id` of the childs element.
fn is_valid_attribute_value(
    attribute: Option<JsxAttribute>,
    jsx_child_list: &JsxChildList,
) -> Option<bool> {
    let attribute_value = attribute?.initializer()?.value().ok()?;
    let is_used_attribute = jsx_child_list
        .iter()
        .filter_map(|child| {
            let jsx_element = child.as_jsx_element()?;
            let opening_element = jsx_element.opening_element().ok()?;
            let maybe_attribute = opening_element.find_attribute_by_name("id").ok()?;
            let child_attribute_value = maybe_attribute?.initializer()?.value().ok()?;
            let is_valid = attribute_value.as_static_value()?.text()
                == child_attribute_value.as_static_value()?.text();
            Some(is_valid)
        })
        .any(|x| x);
    Some(is_used_attribute)
}

/// Checks if the given `JsxChildList` has a valid `title` element.
fn has_valid_title_element(jsx_child_list: &JsxChildList) -> Option<bool> {
    jsx_child_list
        .iter()
        .filter_map(|child| {
            let jsx_element = child.as_jsx_element()?;
            let opening_element = jsx_element.opening_element().ok()?;
            let name = opening_element.name().ok()?;
            let name = name.as_jsx_name()?.value_token().ok()?;
            let has_title_name = name.text_trimmed() == "title";
            if !has_title_name {
                return has_valid_title_element(&jsx_element.children());
            }
            let is_empty_child = jsx_element.children().is_empty();
            Some(has_title_name && !is_empty_child)
        })
        .next()
}
