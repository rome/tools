use crate::builders::{format_inserted, format_only_if_breaks};
use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{JsSyntaxKind, TsIntersectionTypeFields};
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
                    separator: JsSyntaxKind::AMP,
                    leading_separator: leading_separator_token.as_ref()
                },
                types.format()
            )))]
        )
    }
}

pub(crate) struct FormatTypeSetLeadingSeparator<'a> {
    pub(crate) separator: JsSyntaxKind,
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
                    // Complicated: Space depends on whatever it prints the break variant and if there's a trailing comment
                    // if breaks: Fine as is, because it takes the kind of the current token
                    // if flat: Needs to do format_removed which formats the comments in place and uses the previous token to decide on spacing
                    // format_if_breaks(token, content) -> Could handle this logic.
                    [format_only_if_breaks(
                        token,
                        &format_args!(token.format(), space_token())
                    )]
                )
            }
            None => write!(
                f,
                [if_group_breaks(&format_args![
                    format_inserted(self.separator),
                    space_token()
                ])]
            ),
        }
    }
}
