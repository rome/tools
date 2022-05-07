use crate::prelude::*;

use rome_js_syntax::JsAssignmentWithDefault;
use rome_js_syntax::JsAssignmentWithDefaultFields;

impl FormatNode for JsAssignmentWithDefault {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsAssignmentWithDefaultFields {
            pattern,
            eq_token,
            default,
        } = self.as_fields();

        formatted![
            formatter,
            pattern.format(formatter)?,
            space_token(),
            eq_token.format(formatter)?,
            space_token(),
            default.format(formatter)?,
        ]
    }
}
