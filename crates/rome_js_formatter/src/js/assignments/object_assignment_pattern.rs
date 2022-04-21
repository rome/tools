use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsObjectAssignmentPattern;
use rome_js_syntax::JsObjectAssignmentPatternFields;

impl FormatNode for JsObjectAssignmentPattern {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternFields {
            l_curly_token,
            properties,
            r_curly_token,
        } = self.as_fields();

        formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            properties.format(formatter)?,
            &r_curly_token?,
        )
    }
}
