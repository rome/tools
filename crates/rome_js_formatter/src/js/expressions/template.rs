use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsTemplate;
use rome_js_syntax::JsTemplateFields;

impl FormatNodeFields<JsTemplate> for FormatNodeRule<JsTemplate> {
    fn format_fields(node: &JsTemplate, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let JsTemplateFields {
            tag,
            type_arguments,
            l_tick_token,
            elements,
            r_tick_token,
        } = node.as_fields();

        let l_tick = l_tick_token.format();
        let r_tick = r_tick_token.format();

        formatted![
            formatter,
            [
                tag.format(),
                type_arguments.format(),
                line_suffix_boundary(),
                l_tick,
                concat_elements(formatter.format_all(elements.iter().formatted())?),
                r_tick
            ]
        ]
    }
}
