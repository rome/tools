use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsNamespaceImportSpecifier;

impl ToFormatElement for JsNamespaceImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let star = formatter.format_token(&self.star_token()?)?;
        let as_token = formatter.format_token(&self.as_token()?)?;
        let local_name = formatter.format_node(self.local_name()?)?;

        Ok(format_elements![
            star,
            space_token(),
            as_token,
            space_token(),
            local_name
        ])
    }
}
