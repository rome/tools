use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsTypeAliasStatement;

impl ToFormatElement for TsTypeAliasStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let type_token = self.type_token().format(formatter)?;
        let binding_identifier = self.binding_identifier().format(formatter)?;
        let type_parameters = self.type_parameters().format_or_empty(formatter)?;
        let equal_token = self.eq_token().format(formatter)?;
        let r#type = self.ty().format(formatter)?;
        Ok(format_elements![
            type_token,
            space_token(),
            binding_identifier,
            type_parameters,
            space_token(),
            equal_token,
            space_token(),
            r#type
        ])
    }
}
