use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsTemplateChunkElement;

impl ToFormatElement for JsTemplateChunkElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_token(&self.template_chunk_token()?)
    }
}
