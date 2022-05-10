use crate::{
    formatted, space_token, Format, FormatElement, FormatNode, Formatter,
    JsFormatter,
};
use rome_formatter::FormatResult;

use rome_js_syntax::JsImportAssertion;
use rome_js_syntax::JsImportAssertionFields;

impl FormatNode for JsImportAssertion {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportAssertionFields {
            assert_token,
            l_curly_token,
            assertions,
            r_curly_token,
        } = self.as_fields();

        let assert_token = assert_token.format(formatter)?;
        let assertions = assertions.format(formatter)?;

        let result = formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            assertions,
            &r_curly_token?,
        )?;

        formatted![formatter, assert_token, space_token(), result]
    }
}
