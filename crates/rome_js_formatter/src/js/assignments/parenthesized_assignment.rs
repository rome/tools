use crate::{format_elements, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsParenthesizedAssignment;
use rome_js_syntax::JsParenthesizedAssignmentFields;

impl FormatNode for JsParenthesizedAssignment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsParenthesizedAssignmentFields {
            l_paren_token,
            assignment,
            r_paren_token,
        } = self.as_fields();

        Ok(format_elements![
            l_paren_token.format(formatter)?,
            assignment.format(formatter)?,
            r_paren_token.format(formatter)?,
        ])
    }
}
