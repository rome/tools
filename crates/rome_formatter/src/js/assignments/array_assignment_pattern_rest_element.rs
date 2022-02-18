use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsArrayAssignmentPatternRestElement;
use rslint_parser::ast::JsArrayAssignmentPatternRestElementFields;

impl ToFormatElement for JsArrayAssignmentPatternRestElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
