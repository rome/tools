use crate::{
    format_element::{normalize_newlines, Token},
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsTemplateChunkElement;

impl ToFormatElement for JsTemplateChunkElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        // Per https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-static-semantics-trv:
        // In template literals, the '\r' and '\r\n' line terminators are normalized to '\n'
        let chunk = self.template_chunk_token()?;
        formatter.format_replaced(
            &chunk,
            FormatElement::from(Token::new_dynamic(
                normalize_newlines(chunk.text_trimmed(), false),
                chunk.text_trimmed_range(),
            )),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{format, FormatOptions};
    use rslint_parser::parse_script;

    #[test]
    fn template_line_breaks() {
        const CODE: &str = "
            const CR = `\r`;
            const LF = `\n`;
            const CR_LF = `\r\n`;
            const LS = `\u{2028}`;
            const PS = `\u{2029}`;
        ";

        let tree = parse_script(CODE, 0).syntax();
        let result = format(FormatOptions::default(), &tree).unwrap();
        assert!(!result.into_code().contains('\r'));
    }
}
