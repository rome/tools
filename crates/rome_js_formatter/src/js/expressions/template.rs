use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsTemplate;
use rome_js_syntax::JsTemplateFields;

impl FormatNodeFields<JsTemplate> for FormatNodeRule<JsTemplate> {
    fn fmt_fields(node: &JsTemplate, f: &mut JsFormatter) -> FormatResult<()> {
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
            ]
        ]?;

        f.join().entries(elements.iter().formatted()).finish()?;

        write!(f, [r_tick_token.format()])
    }
}
