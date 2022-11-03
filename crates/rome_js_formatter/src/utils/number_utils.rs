use crate::prelude::*;
use crate::utils::string_utils::ToAsciiLowercaseCow;
use crate::JsFormatContext;
use crate::JsFormatter;
use rome_formatter::trivia::format_replaced;
use rome_formatter::Format;
use rome_formatter::FormatResult;
use rome_js_syntax::JsSyntaxKind::JS_NUMBER_LITERAL;
use rome_js_syntax::JsSyntaxKind::TS_NUMBER_LITERAL_TYPE;
use rome_js_syntax::JsSyntaxToken;
use std::borrow::Cow;
use std::num::NonZeroUsize;

pub struct CleanedNumberLiteralText<'token> {
    token: &'token JsSyntaxToken,
    text: Cow<'token, str>,
}

impl Format<JsFormatContext> for CleanedNumberLiteralText<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        format_replaced(
            self.token,
            &syntax_token_cow_slice(
                self.text.clone(),
                self.token,
                self.token.text_trimmed_range().start(),
            ),
        )
        .fmt(f)
    }
}

impl<'token> CleanedNumberLiteralText<'token> {
    pub fn from_number_literal_token(token: &'token JsSyntaxToken) -> Self {
        debug_assert!(matches!(
            &token.kind(),
            JS_NUMBER_LITERAL | TS_NUMBER_LITERAL_TYPE
        ));
        CleanedNumberLiteralText {
            token: &token,
            text: format_trimmed_number(token.text_trimmed()),
        }
    }
}

struct FormatNumberLiteralState {
    curr: Option<(usize, char)>,
    location: FormatNumberLiteralLocation,
}
enum FormatNumberLiteralLocation {
    InIntegerPart,
    InDecimalPart(FormatNumberLiteralDecimalPart),
    InExponent(FormatNumberLiteralExponent),
}
use FormatNumberLiteralLocation::*;
struct FormatNumberLiteralDecimalPart {
    dot_index: usize,
    last_non_zero_index: Option<NonZeroUsize>,
}
struct FormatNumberLiteralExponent {
    e_index: usize,
    is_negative: bool,
    first_digit_index: Option<NonZeroUsize>,
    first_non_zero_index: Option<NonZeroUsize>,
}
// Regex-free version of https://github.com/prettier/prettier/blob/ca246afacee8e6d5db508dae01730c9523bbff1d/src/common/util.js#L341-L356
fn format_trimmed_number(text: &str) -> Cow<str> {
    let text = text.to_ascii_lowercase_cow();
    let mut copied_or_ignored_chars = 0usize;
    let mut iter = text.chars().enumerate();
    let mut state = FormatNumberLiteralState {
        curr: iter.next(),
        location: InIntegerPart,
    };
    // Will be filled only if and when the first place that needs reformatting is detected.
    let mut cleaned_text = String::new();

    // Look at only the start of the text, ignore any sign, and make sure numbers always start with a digit. Add 0 if missing.
    if let Some((_, '+' | '-')) = state.curr {
        state.curr = iter.next();
    }
    if let Some((curr_index, '.')) = state.curr {
        cleaned_text.push_str(&text[copied_or_ignored_chars..curr_index]);
        copied_or_ignored_chars = curr_index;
        cleaned_text.push('0');
    }

    // Loop over the rest of the text, applying the remaining rules.
    // Treating the end of string like another character instead of dealing with None will simplify matching a lot.
    let null_terminator = (text.len(), '\0');
    loop {
        // Look for termination of the decimal part or exponent and see if we need to print it differently.
        match (&state.location, state.curr.unwrap_or(null_terminator)) {
            (
                InDecimalPart(FormatNumberLiteralDecimalPart {
                    dot_index,
                    last_non_zero_index: None,
                }),
                (curr_index, 'e' | '\0'),
            ) => {
                // The decimal part equals zero, ignore it completely.
                // Caveat: Prettier still prints a single `.0` unless there was *only* a trailing dot.
                if curr_index > dot_index + 1 {
                    cleaned_text.push_str(&text[copied_or_ignored_chars..=*dot_index]);
                    cleaned_text.push('0');
                } else {
                    cleaned_text.push_str(&text[copied_or_ignored_chars..*dot_index]);
                }
                copied_or_ignored_chars = curr_index;
            }
            (
                InDecimalPart(FormatNumberLiteralDecimalPart {
                    last_non_zero_index: Some(last_non_zero_index),
                    ..
                }),
                (curr_index, 'e' | '\0'),
            ) if last_non_zero_index.get() < curr_index - 1 => {
                // The decimal part ends with at least one zero, ignore them but copy the part from the dot until the last non-zero.
                cleaned_text.push_str(&text[copied_or_ignored_chars..=last_non_zero_index.get()]);
                copied_or_ignored_chars = curr_index;
            }
            (
                InExponent(FormatNumberLiteralExponent {
                    e_index,
                    first_non_zero_index: None,
                    ..
                }),
                (curr_index, '\0'),
            ) => {
                // The exponent equals zero, ignore it completely.
                cleaned_text.push_str(&text[copied_or_ignored_chars..*e_index]);
                copied_or_ignored_chars = curr_index;
            }
            (
                InExponent(FormatNumberLiteralExponent {
                    e_index,
                    is_negative,
                    first_digit_index: Some(first_digit_index),
                    first_non_zero_index: Some(first_non_zero_index),
                }),
                (curr_index, '\0'),
            ) if (first_digit_index.get() > e_index + 1 && !is_negative)
                || (first_non_zero_index.get() > first_digit_index.get()) =>
            {
                // The exponent begins with a plus or at least one zero, ignore them but copy the part from the first non-zero until the end.
                cleaned_text.push_str(&text[copied_or_ignored_chars..=*e_index]);
                if *is_negative {
                    cleaned_text.push('-');
                }
                cleaned_text.push_str(&text[first_non_zero_index.get()..curr_index]);
                copied_or_ignored_chars = curr_index;
            }
            _ => {}
        }

        // Update location info after the current char
        match state {
            // Cases entering or remaining in decimal part
            FormatNumberLiteralState {
                curr: Some((curr_index, '.')),
                ..
            } => {
                state.location = InDecimalPart(FormatNumberLiteralDecimalPart {
                    dot_index: curr_index,
                    last_non_zero_index: None,
                });
            }
            FormatNumberLiteralState {
                location: InDecimalPart(decimal_part),
                curr: Some((curr_index, '1'..='9')),
            } => {
                state.location = InDecimalPart(FormatNumberLiteralDecimalPart {
                    last_non_zero_index: Some(unsafe {
                        // We've already entered InDecimalPart, so curr_index must be >0
                        NonZeroUsize::new_unchecked(curr_index)
                    }),
                    ..decimal_part
                });
            }
            // Cases entering or remaining in exponent
            FormatNumberLiteralState {
                curr: Some((curr_index, 'e')),
                ..
            } => {
                state.location = InExponent(FormatNumberLiteralExponent {
                    e_index: curr_index,
                    is_negative: false,
                    first_digit_index: None,
                    first_non_zero_index: None,
                });
            }
            FormatNumberLiteralState {
                location: InExponent(exponent),
                curr: Some((_, '-')),
            } => {
                state.location = InExponent(FormatNumberLiteralExponent {
                    is_negative: true,
                    ..exponent
                });
            }
            FormatNumberLiteralState {
                location:
                    InExponent(
                        exponent @ FormatNumberLiteralExponent {
                            first_digit_index: None,
                            ..
                        },
                    ),
                curr: Some((curr_index, curr_char @ '0'..='9')),
            } => {
                state.location = InExponent(FormatNumberLiteralExponent {
                    first_digit_index: Some(unsafe {
                        // We've already entered InExponent, so curr_index must be >0
                        NonZeroUsize::new_unchecked(curr_index)
                    }),
                    first_non_zero_index: if curr_char != '0' {
                        Some(unsafe {
                            // We've already entered InExponent, so curr_index must be >0
                            NonZeroUsize::new_unchecked(curr_index)
                        })
                    } else {
                        None
                    },
                    ..exponent
                });
            }
            FormatNumberLiteralState {
                location:
                    InExponent(
                        exponent @ FormatNumberLiteralExponent {
                            first_non_zero_index: None,
                            ..
                        },
                    ),
                curr: Some((curr_index, '1'..='9')),
            } => {
                state.location = InExponent(FormatNumberLiteralExponent {
                    first_non_zero_index: Some(unsafe { NonZeroUsize::new_unchecked(curr_index) }),
                    ..exponent
                });
            }
            _ => {}
        }

        // Repeat or exit
        match state.curr {
            None | Some((_, 'x') /* hex bailout */) => break,
            Some(_) => state.curr = iter.next(),
        }
    }

    if cleaned_text.is_empty() {
        text
    } else {
        // Append any unconsidered text
        cleaned_text.push_str(&text[copied_or_ignored_chars..]);
        Cow::Owned(cleaned_text)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::format_trimmed_number;

    #[test]
    fn removes_unnecessary_plus_and_zeros_from_scientific_notation() {
        assert_eq!("1e2", format_trimmed_number("1e02"));
        assert_eq!("1e2", format_trimmed_number("1e+2"));
    }

    #[test]
    fn removes_unnecessary_scientific_notation() {
        assert_eq!("1", format_trimmed_number("1e0"));
        assert_eq!("1", format_trimmed_number("1e-0"));
    }
    #[test]
    fn does_not_get_bamboozled_by_hex() {
        assert_eq!("0xe0", format_trimmed_number("0xe0"));
        assert_eq!("0x10e0", format_trimmed_number("0x10e0"));
    }

    #[test]
    fn makes_sure_numbers_always_start_with_a_digit() {
        assert_eq!("0.2", format_trimmed_number(".2"));
    }

    #[test]
    fn removes_extraneous_trailing_decimal_zeroes() {
        assert_eq!("0.1", format_trimmed_number("0.10"));
    }
    #[test]
    fn keeps_one_trailing_decimal_zero() {
        assert_eq!("0.0", format_trimmed_number("0.00"));
    }

    #[test]
    fn removes_trailing_dot() {
        assert_eq!("1", format_trimmed_number("1."));
    }

    #[test]
    fn cleans_all_at_once() {
        assert_eq!("0.0", format_trimmed_number(".00e-0"));
    }

    #[test]
    fn keeps_the_input_string_if_no_change_needed() {
        assert!(matches!(
            format_trimmed_number("0.1e2"),
            Cow::Borrowed("0.1e2")
        ));
    }
}
