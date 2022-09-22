use crate::prelude::*;

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

        write!(f, [l_brack_token.format(),])?;

        if elements.is_empty() {
            write!(
                f,
                [format_dangling_comments(node.syntax()).with_block_indent()]
            )?;
        } else {
            write!(f, [group(&soft_block_indent(&elements.format()))])?;
        }

        write!(f, [r_brack_token.format()])
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsArrayBindingPattern,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled inside of `fmt_fields`
        Ok(())
    }
}
