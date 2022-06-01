use crate::prelude::*;
use crate::utils::format_with_semicolon;
use crate::FormatNodeFields;
use rome_js_syntax::{TsMappedType, TsMappedTypeFields};

impl FormatNodeFields<TsMappedType> for FormatNodeRule<TsMappedType> {
    fn format_fields(node: &TsMappedType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsMappedTypeFields {
            l_curly_token,
            readonly_modifier,
            l_brack_token,
            property_name,
            in_token,
            keys_type,
            as_clause,
            r_brack_token,
            optional_modifier,
            mapped_type,
            semicolon_token,
            r_curly_token,
        } = node.as_fields();

        formatter
            .delimited(
                &l_curly_token?,
                format_with_semicolon(
                    formatter,
                    formatted![
                        formatter,
                        [
                            readonly_modifier
                                .format()
                                .with_or_empty(|readonly| formatted![
                                    formatter,
                                    [readonly, space_token()]
                                ]),
                            l_brack_token.format(),
                            property_name.format(),
                            space_token(),
                            in_token.format(),
                            space_token(),
                            keys_type.format(),
                            as_clause.format().with_or_empty(|clause| formatted![
                                formatter,
                                [space_token(), clause]
                            ]),
                            r_brack_token.format(),
                            optional_modifier.format(),
                            mapped_type.format(),
                        ]
                    ]?,
                    semicolon_token,
                )?,
                &r_curly_token?,
            )
            .block_indent()
            .finish()
    }
}
