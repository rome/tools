use crate::{
    format_elements, soft_line_break_or_space, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsNamedImportSpecifier;

impl ToFormatElement for JsNamedImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = formatter.format_node(self.name()?)?;
        let as_token = formatter.format_token(&self.as_token()?)?;
        let local_name = formatter.format_node(self.local_name()?)?;

        Ok(format_elements![
            name,
            soft_line_break_or_space(),
            as_token,
            soft_line_break_or_space(),
            local_name
        ])
    }
}
