use crate::format_traits::FormatOptional;
use crate::utils::format_with_semicolon;
use crate::{space_token, Format, FormatElement, FormatNode, Formatter, JsFormatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{TsMappedType, TsMappedTypeFields};

impl FormatNode for TsMappedType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
        } = self.as_fields();

        let readonly = readonly_modifier
            .with_or_empty(|readonly| formatted![formatter, readonly, space_token()]);
        let l_square = l_brack_token.format(formatter)?;
        let property_name = property_name.format(formatter)?;
        let in_token = in_token.format(formatter)?;
        let keys = keys_type.format(formatter)?;
        let as_clause =
            as_clause.with_or_empty(|clause| formatted![formatter, space_token(), clause]);
        let r_square = r_brack_token.format(formatter)?;

        formatter.format_delimited_block_indent(
            &l_curly_token?,
            format_with_semicolon(
                formatter,
                formatted![
                    formatter,
                    readonly,
                    l_square,
                    property_name,
                    space_token(),
                    in_token,
                    space_token(),
                    keys,
                    as_clause,
                    r_square,
                    optional_modifier,
                    mapped_type,
                ]?,
                semicolon_token,
            )?,
            &r_curly_token?,
        )
    }
}
