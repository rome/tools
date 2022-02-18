use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, soft_line_break_or_space, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsNamedImportSpecifier;
use rslint_parser::ast::JsNamedImportSpecifierFields;

impl ToFormatElement for JsNamedImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNamedImportSpecifierFields {
            type_token,
            name,
            as_token,
            local_name,
        } = self.as_fields();

        let name = name.format(formatter)?;
        let as_token = as_token.format(formatter)?;
        let local_name = local_name.format(formatter)?;

        Ok(format_elements![
            name,
            soft_line_break_or_space(),
            as_token,
            soft_line_break_or_space(),
            local_name
        ])
    }
}
