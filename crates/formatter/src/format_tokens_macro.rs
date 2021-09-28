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
/// use rome_formatter::{format_tokens, FormatToken};
/// let token = format_tokens!("foo:", FormatToken::Space, "bar");
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
/// use rome_formatter::{format_tokens, format_token, IndentToken, FormatToken, FormatOptions};
/// let token = format_tokens!(
///   "foo:",
///   FormatToken::Space,
///   "{",
///   IndentToken::new(
///     format_tokens!(FormatToken::Space, "bar:", FormatToken::Space, "lorem")
///   ),
///    FormatToken::Space,
///   "}"
/// );
/// assert_eq!(r#"foo: { bar: lorem }"#, format_token(&token, FormatOptions::default()).code());
/// ```
/// Or you can also create single tokens:
/// ```
/// use rome_formatter::{format_tokens, format_token, FormatOptions};
/// let unique_token = format_tokens!("single");
/// assert_eq!(r#"single"#, format_token(&unique_token, FormatOptions::default()).code());
/// ```
#[macro_export]
macro_rules! format_tokens {

	// called for things like format_tokens!("hey")
	($token:expr) => {
		{
			use $crate::FormatToken;
			FormatToken::from($token)
		}
	};

	( $( $token:expr ),+ $(,)?) => {{
		use $crate::{FormatToken, ListToken};
		FormatToken::concat(vec![
			$(
					 FormatToken::from($token)
			),+

		])
	}};
}
