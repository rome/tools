use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsConstructSignatureTypeMember;

impl ToFormatElement for TsConstructSignatureTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let new = self.new_token().format(formatter)?;
        let type_parameters = self.type_parameters().format_or_empty(formatter)?;
        let parameters = self.parameters().format(formatter)?;
        let type_annotation = self.type_annotation().format_or_empty(formatter)?;

        Ok(format_elements![
            new,
            type_parameters,
            parameters,
            type_annotation
        ])
    }
}
