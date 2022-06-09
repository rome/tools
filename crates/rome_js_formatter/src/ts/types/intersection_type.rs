use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsIntersectionTypeFields;
use rome_js_syntax::{JsSyntaxToken, TsIntersectionType};

impl FormatNodeFields<TsIntersectionType> for FormatNodeRule<TsIntersectionType> {
    fn fmt_fields(node: &TsIntersectionType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIntersectionTypeFields {
            leading_separator_token,
            types,
        } = node.as_fields();

        write!(
            f,
            [group_elements(&indent(&format_args!(
                soft_line_break(),
                FormatTypeSetLeadingSeparator {
                    separator: "&",
                    leading_separator: leading_separator_token.as_ref()
                },
                types.format()
            )))]
        )
    }
}

pub(crate) struct FormatTypeSetLeadingSeparator<'a> {
    pub(crate) separator: &'static str,
    pub(crate) leading_separator: Option<&'a JsSyntaxToken>,
}

impl Format<JsFormatContext> for FormatTypeSetLeadingSeparator<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match &self.leading_separator {
            Some(token) => {
                // The SyntaxToken is converted into a FormatElement using
                // Token::from to strip the token's trivia pieces which are
                // then reinserted in format_replaced outside of the
                // if_group_breaks block to avoid removing comments when the
                // group does not break
                write!(
                    f,
                    [format_replaced(
                        token,
                        &if_group_breaks(&format_args!(format_trimmed_token(token), space_token()))
                    )]
                )
            }
            None => write!(
                f,
                [if_group_breaks(&format_args![
                    token(self.separator),
                    space_token()
                ])]
            ),
        }
    }
}
