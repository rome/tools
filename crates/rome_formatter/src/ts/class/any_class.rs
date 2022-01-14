use crate::{
    block_indent, empty_element, format_elements, group_elements, hard_line_break, join_elements,
    space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsAnyClass;

impl ToFormatElement for JsAnyClass {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let id = match self.id()? {
            Some(id) => format_elements![space_token(), formatter.format_node(id)?],
            None => empty_element(),
        };

        let extends = match self.extends_clause() {
            Some(extends_clause) => {
                format_elements![space_token(), formatter.format_node(extends_clause)?]
            }
            None => empty_element(),
        };

        Ok(format_elements![
            formatter.format_token(&self.class_token()?)?,
            id,
            extends,
            space_token(),
            group_elements(format_elements![
                formatter.format_token(&self.l_curly_token()?)?,
                block_indent(join_elements(
                    hard_line_break(),
                    formatter.format_nodes(self.members())?
                )),
                formatter.format_token(&self.r_curly_token()?)?
            ])
        ])
    }
}
