use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::{JsxAttributeInitializerClause, JsxAttributeInitializerClauseFields};

impl FormatNode for JsxAttributeInitializerClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxAttributeInitializerClauseFields { eq_token, value } = self.as_fields();

        Ok(format_elements![
            eq_token.format(formatter)?,
            value.format(formatter)?
        ])
    }
}
