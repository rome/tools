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
/// use rome_formatter::{format_tokens, FormatToken, Tokens, format_token, FormatOptions};
///
/// let mut tokens = Tokens::default();
/// let token = format_tokens!(
///     tokens.double_quoted_string("foo"),
///     tokens.colon(),
///     FormatToken::Space,
///     tokens.double_quoted_string("bar")
/// );
///
/// assert_eq!(
///     r#""foo": "bar""#,
///     format_token(&token, FormatOptions::default()).root().text().to_string().as_str()
/// )
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
/// use rome_formatter::{format_tokens, format_token, IndentToken, FormatToken, FormatOptions, Tokens};
///
/// let mut tokens = Tokens::default();
///
/// let token = format_tokens!(
///   tokens.double_quoted_string("foo"),
///   tokens.colon(),
///   FormatToken::Space,
///   tokens.left_brace(),
///   FormatToken::Space,
///   IndentToken::new(
///     format_tokens![
///       tokens.double_quoted_string("bar"),
///       tokens.colon(),
///       FormatToken::Space,
///       tokens.double_quoted_string("lorem"),
///     ]
///   ),
///   FormatToken::Space,
///   tokens.right_brace()
/// );
///
/// assert_eq!(
///   r#""foo": { "bar": "lorem" }"#,
///   format_token(&token, FormatOptions::default()).root().text().to_string().as_str());
/// ```
/// Or you can also create single tokens:
/// ```
/// use rome_formatter::{format_tokens, format_token, FormatOptions, Tokens};
/// let mut tokens = Tokens::default();
///
/// let unique_token = format_tokens!(tokens.double_quoted_string("single"));
/// assert_eq!(
///   r#""single""#,
///   format_token(&unique_token, FormatOptions::default()).root().text().to_string().as_str()
/// );
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
		use $crate::{FormatToken};
		FormatToken::concat(vec![
			$(
					 FormatToken::from($token)
			),+

		])
	}};
}
