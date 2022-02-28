use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsModuleDeclaration;
use rslint_parser::ast::TsModuleDeclarationFields;

impl ToFormatElement for TsModuleDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsModuleDeclarationFields {
            module_or_namespace,
            name,
            body,
        } = self.as_fields();

        Ok(format_elements![
            module_or_namespace.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            space_token(),
            body.format(formatter)?,
        ])
    }
}
