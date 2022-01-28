use crate::{
    empty_element, format_element::join_elements_hard_line, format_elements, join_elements,
    space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsTemplate;

impl ToFormatElement for JsTemplate {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let tag = if let Some(tag) = self.tag() {
            formatter.format_node(tag)?
        } else {
            empty_element()
        };
        let l_tick = formatter.format_token(&self.l_tick_token()?)?;
        let r_tick = formatter.format_token(&self.r_tick_token()?)?;

        Ok(format_elements![
            tag,
            l_tick,
            join_elements(empty_element(), formatter.format_nodes(self.elements())?),
            r_tick
        ])
    }
}
