use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsObjectBindingPattern;
use rome_js_syntax::JsObjectBindingPatternFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsObjectBindingPattern;

impl FormatNodeRule<JsObjectBindingPattern> for FormatJsObjectBindingPattern {
    fn fmt_fields(node: &JsObjectBindingPattern, f: &mut JsFormatter) -> FormatResult<()> {
        let JsObjectBindingPatternFields {
            l_curly_token,
            properties,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_delimited(&l_curly_token?, &properties.format(), &r_curly_token?,)
                    .soft_block_spaces()
            ]
        )
    }
}
