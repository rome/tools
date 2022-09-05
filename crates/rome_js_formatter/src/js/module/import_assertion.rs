use crate::prelude::*;

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
            write!(f, [l_curly_token.format(), r_curly_token.format()])
        } else {
            write!(
                f,
                [
                    format_delimited(&l_curly_token?, &assertions.format(), &r_curly_token?,)
                        .soft_block_spaces()
                ]
            )
        }
    }
}
