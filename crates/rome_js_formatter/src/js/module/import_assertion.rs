use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsImportAssertion;
use rome_js_syntax::JsImportAssertionFields;

impl FormatNodeFields<JsImportAssertion> for FormatNodeRule<JsImportAssertion> {
    fn format_fields(
        node: &JsImportAssertion,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsImportAssertionFields {
            assert_token,
            l_curly_token,
            assertions,
            r_curly_token,
        } = node.as_fields();

        let result = formatter
            .delimited(
                &l_curly_token?,
                formatted![formatter, [assertions.format()]]?,
                &r_curly_token?,
            )
            .soft_block_spaces()
            .finish()?;

        formatted![formatter, [assert_token.format(), space_token(), result]]
    }
}
