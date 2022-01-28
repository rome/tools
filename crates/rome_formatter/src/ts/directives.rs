use crate::{
    format_elements, join_elements_hard_line, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::{AstNode, AstNodeList, JsDirective, JsDirectiveList};

pub fn format_directives(directives: JsDirectiveList, formatter: &Formatter) -> FormatElement {
    join_elements_hard_line(directives.iter().map(|directive| {
        let snapshot = formatter.snapshot();
        let elem = match formatter.format_node(&directive) {
            Ok(result) => result,
            Err(_) => {
                formatter.restore(snapshot);
                formatter
                    .format_verbatim(directive.syntax())
                    .trim_start()
                    .trim_end()
            }
        };

        (directive, elem)
    }))
}

impl ToFormatElement for JsDirective {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.value_token()?)?,
            formatter
                .format_token(&self.semicolon_token())?
                .unwrap_or_else(|| token(';')),
        ])
    }
}
