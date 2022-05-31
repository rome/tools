use crate::formatter::FormatTrimmedToken;
use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write, Buffer, VecBuffer};
use rome_js_syntax::TsUnionType;
use rome_js_syntax::TsUnionTypeFields;

impl FormatNodeFields<TsUnionType> for FormatNodeRule<TsUnionType> {
    fn format_fields(node: &TsUnionType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsUnionTypeFields {
            leading_separator_token,
            types,
        } = node.as_fields();

        let _leading_separator_token = format_once(|f| {
            match leading_separator_token {
                Some(token) => {
                    // The SyntaxToken is converted into a FormatElement using
                    // Token::from to strip the token's trivia pieces which are
                    // then reinserted informat_replaced outside of the
                    // if_group_breaks block to avoid removing comments when the
                    // group does not break
                    write!(
                        f,
                        [f.format_replaced(
                            &token,
                            &if_group_breaks(&format_args!(
                                FormatTrimmedToken::new(&token),
                                space_token()
                            ))
                        )]
                    )
                }
                None => write!(
                    f,
                    [if_group_breaks(&format_args![token("|"), space_token()])]
                ),
            }
        });

        let mut buffer = VecBuffer::new(f.state_mut());
        write!(buffer, [types.format()])?;

        let types = buffer.into_element();

        // Push trailing comments for the union out of the group (and indent block),
        // so any potential line break doesn't influence the formatting of the type itself
        let (leading_comments, types, trailing_comments) = types.split_trivia();

        write![
            f,
            [
                group_elements(&indent(&format_args![
                    soft_line_break(),
                    format_once(|f| {
                        f.write_element(leading_comments);
                        f.write_element(types);
                        Ok(())
                    })
                ])),
                format_once(|f| {
                    f.write_element(trailing_comments);
                    Ok(())
                })
            ]
        ]
    }
}
