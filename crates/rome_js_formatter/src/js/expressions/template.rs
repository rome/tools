use crate::prelude::*;

use rome_js_syntax::JsTemplate;
use rome_js_syntax::JsTemplateFields;

impl FormatNode for JsTemplate {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsTemplateFields {
            tag,
            type_arguments,
            l_tick_token,
            elements,
            r_tick_token,
        } = self.as_fields();

        let l_tick = l_tick_token.format(formatter)?;
        let r_tick = r_tick_token.format(formatter)?;

        Ok(hard_group_elements(formatted![
            formatter,
            tag,
            type_arguments,
            l_tick,
            concat_elements(formatter.format_all(elements)?),
            r_tick
        ]?))
    }
}
