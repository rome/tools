use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_with_semicolon;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::{TsMappedType, TsMappedTypeFields};

impl ToFormatElement for TsMappedType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsMappedTypeFields {
            l_curly_token,
            readonly_modifier,
            l_brack_token,
            property_name,
            in_token,
            keys_type,
            as_clause: _,
            r_brack_token,
            optional_modifier,
            mapped_type,
            semicolon_token,
            r_curly_token,
        } = self.as_fields();

        let readonly = readonly_modifier.format_with_or_empty(formatter, |readonly| {
            format_elements![readonly, space_token()]
        })?;
        let l_square = l_brack_token.format(formatter)?;
        let property_name = property_name.format(formatter)?;
        let in_token = in_token.format(formatter)?;
        let keys = keys_type.format(formatter)?;
        let as_clause = self
            .as_clause()
            .format_with_or_empty(formatter, |clause| format_elements![space_token(), clause])?;
        let r_square = r_brack_token.format(formatter)?;
        let optional_modifier = optional_modifier.format_or_empty(formatter)?;
        let mapped_type = mapped_type.format_or_empty(formatter)?;

        formatter.format_delimited_block_indent(
            &l_curly_token?,
            format_with_semicolon(
                formatter,
                format_elements![
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
                ],
                semicolon_token,
            )?,
            &r_curly_token?,
        )
    }
}
