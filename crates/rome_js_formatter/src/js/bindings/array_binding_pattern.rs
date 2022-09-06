use crate::prelude::*;

use crate::builders::format_delimited;
use rome_formatter::write;
use rome_js_syntax::JsArrayBindingPattern;
use rome_js_syntax::JsArrayBindingPatternFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrayBindingPattern;

impl FormatNodeRule<JsArrayBindingPattern> for FormatJsArrayBindingPattern {
    fn fmt_fields(&self, node: &JsArrayBindingPattern, f: &mut JsFormatter) -> FormatResult<()> {
        let JsArrayBindingPatternFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_delimited(&l_brack_token?, &elements.format(), &r_brack_token?,)
                    .soft_block_indent()
            ]
        )
    }
}
