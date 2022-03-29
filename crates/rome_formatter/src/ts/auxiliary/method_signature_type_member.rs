use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_type_member_separator;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsMethodSignatureTypeMember;

impl ToFormatElement for TsMethodSignatureTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        let optional_token = self.optional_token().format_or_empty(formatter)?;
        let type_arguments = self.type_parameters().format_or_empty(formatter)?;
        let parameters = self.parameters().format(formatter)?;
        let return_type_annotation = self.return_type_annotation().format_or_empty(formatter)?;
        let separator = format_type_member_separator(self.separator_token(), formatter);
        Ok(format_elements![
            name,
            optional_token,
            type_arguments,
            parameters,
            return_type_annotation,
            separator
        ])
    }
}
