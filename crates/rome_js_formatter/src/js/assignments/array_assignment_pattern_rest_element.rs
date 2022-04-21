use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsArrayAssignmentPatternRestElement;
use rome_js_syntax::JsArrayAssignmentPatternRestElementFields;

impl FormatNode for JsArrayAssignmentPatternRestElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsArrayAssignmentPatternRestElementFields {
            dotdotdot_token,
            pattern,
        } = self.as_fields();

        Ok(format_elements![
            dotdotdot_token.format(formatter)?,
            pattern.format(formatter)?
        ])
    }
}
