use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_object_type_member_separator;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsPropertySignatureTypeMember;

impl ToFormatElement for TsPropertySignatureTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let readonly = self.readonly_token().format_or_empty(formatter)?;
        let name = self.name().format(formatter)?;
        let type_annotation = self.type_annotation().format_or_empty(formatter)?;
        let optional = self.optional_token_token().format_or_empty(formatter)?;
        let separator = format_object_type_member_separator(self.separator_token(), formatter)?;

        Ok(format_elements![
            readonly,
            space_token(),
            name,
            optional,
            type_annotation,
            separator
        ])
    }
}
