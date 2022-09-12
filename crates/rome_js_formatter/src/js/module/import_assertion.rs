use crate::prelude::*;

use crate::builders::format_delimited;
use rome_formatter::write;
use rome_js_syntax::JsImportAssertion;
use rome_js_syntax::JsImportAssertionFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsImportAssertion;

impl FormatNodeRule<JsImportAssertion> for FormatJsImportAssertion {
    fn fmt_fields(&self, node: &JsImportAssertion, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportAssertionFields {
            assert_token,
            l_curly_token,
            assertions,
            r_curly_token,
        } = node.as_fields();

        write![f, [assert_token.format(), space()]]?;

        if assertions.is_empty() {
            let has_dangling = f.comments().has_dangling_comments(node.syntax());
            write!(
                f,
                [
                    l_curly_token.format(),
                    has_dangling.then_some(space()),
                    format_dangling_comments(node.syntax()).with_soft_block_indent(),
                    has_dangling.then_some(space()),
                    r_curly_token.format(),
                ]
            )
        } else {
            write!(
                f,
                [
                    format_delimited(&l_curly_token?, &assertions.format(), &r_curly_token?)
                        .soft_block_spaces()
                ]
            )
        }
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsImportAssertion,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled as part of `fmt_fields`
        Ok(())
    }
}
