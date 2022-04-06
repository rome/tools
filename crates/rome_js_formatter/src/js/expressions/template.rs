use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, hard_group_elements, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsTemplate;
use rome_js_syntax::JsTemplateFields;

impl ToFormatElement for JsTemplate {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsTemplateFields {
            tag,
            type_arguments,
            l_tick_token,
            elements,
            r_tick_token,
        } = self.as_fields();

        let tag = tag.format_or_empty(formatter)?;
        let type_arguments = type_arguments.format_or_empty(formatter)?;
        let l_tick = l_tick_token.format(formatter)?;
        let r_tick = r_tick_token.format(formatter)?;

        Ok(hard_group_elements(format_elements![
            tag,
            type_arguments,
            l_tick,
            concat_elements(formatter.format_nodes(elements)?),
            r_tick
        ]))
    }
}
