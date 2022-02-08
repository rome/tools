use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, soft_line_break_or_space, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsNamedImportSpecifier;

impl ToFormatElement for JsNamedImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        let as_token = self.as_token().format(formatter)?;
        let local_name = self.local_name().format(formatter)?;

        Ok(format_elements![
            name,
            soft_line_break_or_space(),
            as_token,
            soft_line_break_or_space(),
            local_name
        ])
    }
}
