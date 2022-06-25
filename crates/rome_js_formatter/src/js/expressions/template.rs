use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsTemplate;
use rome_js_syntax::JsTemplateFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsTemplate;

impl FormatNodeRule<JsTemplate> for FormatJsTemplate {
    fn fmt_fields(&self, node: &JsTemplate, f: &mut JsFormatter) -> FormatResult<()> {
        let JsTemplateFields {
            tag,
            type_arguments,
            l_tick_token,
            elements,
            r_tick_token,
        } = node.as_fields();

        write![
            f,
            [
                tag.format(),
                type_arguments.format(),
                line_suffix_boundary(),
                l_tick_token.format(),
                elements.format(),
                r_tick_token.format()
            ]
        ]
    }
}
