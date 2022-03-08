use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::JsExportDefaultDeclarationClause;

impl ToFormatElement for JsExportDefaultDeclarationClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.default_token().format(formatter)?,
            space_token(),
            self.declaration().format(formatter)?
        ])
    }
}
