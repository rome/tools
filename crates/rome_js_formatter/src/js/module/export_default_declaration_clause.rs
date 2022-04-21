use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::JsExportDefaultDeclarationClause;

impl FormatNode for JsExportDefaultDeclarationClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.default_token().format(formatter)?,
            space_token(),
            self.declaration().format(formatter)?
        ])
    }
}
