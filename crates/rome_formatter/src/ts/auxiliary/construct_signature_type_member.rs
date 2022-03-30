use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_type_member_separator;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{TsConstructSignatureTypeMember, TsConstructSignatureTypeMemberFields};

impl ToFormatElement for TsConstructSignatureTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsConstructSignatureTypeMemberFields {
            new_token,
            type_parameters,
            parameters,
            type_annotation,
            separator_token,
        } = self.as_fields();

        let new = new_token.format(formatter)?;
        let type_parameters = type_parameters.format_or_empty(formatter)?;
        let parameters = parameters.format(formatter)?;
        let type_annotation = type_annotation.format_or_empty(formatter)?;
        let separator_token = format_type_member_separator(separator_token, formatter);

        Ok(format_elements![
            new,
            type_parameters,
            parameters,
            type_annotation,
            separator_token,
        ])
    }
}
