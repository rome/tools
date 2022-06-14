use crate::prelude::*;
use crate::ts::types::intersection_type::FormatTypeSetLeadingSeparator;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write, Buffer, VecBuffer};
use rome_js_syntax::TsUnionType;
use rome_js_syntax::TsUnionTypeFields;

impl FormatNodeFields<TsUnionType> for FormatNodeRule<TsUnionType> {
    fn fmt_fields(node: &TsUnionType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsUnionTypeFields {
            leading_separator_token,
            types,
        } = node.as_fields();

        let mut buffer = VecBuffer::new(f.state_mut());
        write!(
            buffer,
            [
                FormatTypeSetLeadingSeparator {
                    separator: "|",
                    leading_separator: leading_separator_token.as_ref()
                },
                types.format()
            ]
        )?;

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
                        f.write_element(leading_comments)?;
                        f.write_element(types)
                    })
                ])),
                format_once(|f| { f.write_element(trailing_comments) })
            ]
        ]
    }
}
