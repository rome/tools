use crate::{
    format_elements, hard_line_break, join_elements, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::{AstNode, AstNodeList, JsDirective, JsDirectiveList};

pub fn format_directives(directives: JsDirectiveList, formatter: &Formatter) -> FormatElement {
    join_elements(
        hard_line_break(),
        directives.iter().map(|directive| {
            formatter
                .format_node(directive.clone())
                .unwrap_or_else(|_| {
                    formatter
                        .format_raw(directive.syntax())
                        .trim_start()
                        .trim_end()
                })
        }),
    )
}

impl ToFormatElement for JsDirective {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let semicolon = if let Some(token) = self.semicolon_token() {
            formatter.format_token(&token)?
        } else {
            token(';')
        };

        Ok(format_elements![
            formatter.format_token(&self.value_token()?)?,
            semicolon,
        ])
    }
}
