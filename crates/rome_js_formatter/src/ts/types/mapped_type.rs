use crate::prelude::*;
use crate::utils::FormatWithSemicolon;

use rome_formatter::{format_args, write};
use rome_js_syntax::{TsMappedType, TsMappedTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsMappedType;

impl FormatNodeRule<TsMappedType> for FormatTsMappedType {
    fn fmt_fields(&self, node: &TsMappedType, f: &mut JsFormatter) -> FormatResult<()> {
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

        write!(
            f,
            [format_delimited(
                &l_curly_token?,
                &FormatWithSemicolon::new(
                    &format_args!(
                        readonly_modifier
                            .format()
                            .with_or_empty(|readonly, f| write![f, [readonly, space()]]),
                        l_brack_token.format(),
                        property_name.format(),
                        space(),
                        in_token.format(),
                        space(),
                        keys_type.format(),
                        as_clause
                            .format()
                            .with_or_empty(|clause, f| write![f, [space(), clause]]),
                        r_brack_token.format(),
                        optional_modifier.format(),
                        mapped_type.format(),
                    ),
                    semicolon_token.as_ref(),
                ),
                &r_curly_token?,
            )
            .block_indent()]
        )
    }
}
