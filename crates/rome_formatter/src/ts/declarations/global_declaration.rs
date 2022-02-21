use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsGlobalDeclaration;
use rslint_parser::ast::TsGlobalDeclarationFields;

impl ToFormatElement for TsGlobalDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsGlobalDeclarationFields { global_token, body } = self.as_fields();

        Ok(format_elements![
            global_token.format(formatter)?,
            space_token(),
            body.format(formatter)?
        ])
    }
}
