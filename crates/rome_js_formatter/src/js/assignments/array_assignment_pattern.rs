use crate::{Format, FormatElement, FormatNode, Formatter, JsFormatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsArrayAssignmentPattern;
use rome_js_syntax::JsArrayAssignmentPatternFields;

impl FormatNode for JsArrayAssignmentPattern {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsArrayAssignmentPatternFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = self.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_brack_token?,
            elements.format(formatter)?,
            &r_brack_token?,
        )
    }
}
