use crate::{
    format_element::{normalize_newlines, Token},
    FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsTemplateChunkElement;
use rslint_parser::ast::JsTemplateChunkElementFields;

impl ToFormatElement for JsTemplateChunkElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsTemplateChunkElementFields {
            template_chunk_token,
        } = self.as_fields();

        // Per https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-static-semantics-trv:
        // In template literals, the '\r' and '\r\n' line terminators are normalized to '\n'
        let chunk = template_chunk_token?;
        formatter.format_replaced(
            &chunk,
            FormatElement::from(Token::new_dynamic(
                normalize_newlines(chunk.text_trimmed(), ['\r']).into_owned(),
                chunk.text_trimmed_range(),
            )),
        )
    }
}
