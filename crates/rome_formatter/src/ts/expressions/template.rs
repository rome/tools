use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsTemplate;

impl ToFormatElement for JsTemplate {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let tag = self.tag().format_or_empty(formatter)?;
        let l_tick = self.l_tick_token().format(formatter)?;
        let r_tick = self.r_tick_token().format(formatter)?;

        Ok(format_elements![
            tag,
            l_tick,
            concat_elements(formatter.format_nodes(self.elements())?),
            r_tick
        ])
    }
}
