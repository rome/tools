/// The macro `format_tokens` is a convenience macro to
/// use when writing a list of tokens that should be at the same level
/// without particular rule.
///
/// # Examples
///
/// Let's suppose you need to create tokens for the string `"foo": "bar"`,
/// you would write:
///
/// ```rust
/// use rome_js_formatter::{format_elements, space_token, token};
/// let element = format_elements![token("foo:"), space_token(), token("bar")];
/// ```
///
/// The macro can be also nested, although the macro needs to be decorated with the token you need.
/// For example, let's try to format following string:
///
/// ```no_rust
/// foo: { bar: lorem }
/// ```
/// You would write it like the following:
///
/// ```rust
/// use rome_js_formatter::{format_elements, format_element, FormatOptions, space_token, token};
/// let element = format_elements![
///   token("foo:"),
///   space_token(),
///   token("{"),
///   space_token(),
///   token("bar:"),
///   space_token(),
///   token("lorem"),
///   space_token(),
///   token("}")
/// ];
/// assert_eq!(r#"foo: { bar: lorem }"#, format_element(&element, FormatOptions::default()).as_code());
/// ```
/// Or you can also create single element:
/// ```
/// use rome_js_formatter::{format_elements, format_element, FormatOptions, token};
/// let element = format_elements![token("single")];
/// assert_eq!(r#"single"#, format_element(&element, FormatOptions::default()).as_code());
/// ```
#[macro_export]
macro_rules! format_elements {

    // called for things like format_tokens!["hey"]
    ($element:expr) => {
        {
            use $crate::FormatElement;
            FormatElement::from($element)
        }
    };

    ( $( $element:expr ),+ $(,)?) => {{
        use $crate::{FormatElement, concat_elements};
        concat_elements([
            $(
                     FormatElement::from($element)
            ),+
        ])
    }};
}
