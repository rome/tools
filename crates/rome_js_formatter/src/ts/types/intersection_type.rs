use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsIntersectionType;
use rome_js_syntax::TsIntersectionTypeFields;

impl FormatNodeFields<TsIntersectionType> for FormatNodeRule<TsIntersectionType> {
    fn format_fields(node: &TsIntersectionType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIntersectionTypeFields {
            leading_separator_token,
            types,
        } = node.as_fields();

        let leading_separator_token = format_once(|f| {
            match leading_separator_token {
                Some(token) => {
                    // The SyntaxToken is converted into a FormatElement using
                    // Token::from to strip the token's trivia pieces which are
                    // then reinserted in format_replaced outside of the
                    // if_group_breaks block to avoid removing comments when the
                    // group does not break
                    write!(
                        f,
                        [format_replaced(
                            &token,
                            &if_group_breaks(format_args!(
                                format_trimmed_token(&token),
                                space_token()
                            ))
                        )]
                    )
                }
                None => write!(
                    f,
                    [if_group_breaks(format_args![token("&"), space_token()])]
                ),
            }
        });

        write!(
            f,
            [group_elements(indent(format_args!(
                soft_line_break(),
                leading_separator_token,
                types.format()
            )))]
        )
    }
}
