use crate::{
    empty_element, format_elements, hard_line_break, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::{AstNodeList, JsDirective, JsDirectiveList};

pub fn format_directives_list(directives: JsDirectiveList, formatter: &Formatter) -> FormatElement {
    if !directives.is_empty() {
        format_elements![formatter.format_list(directives), hard_line_break()]
    } else {
        empty_element()
    }
}

impl ToFormatElement for JsDirective {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.value_token()?)?,
            formatter
                .format_token(&self.semicolon_token())?
                .unwrap_or_else(|| token(";")),
        ])
    }
}
