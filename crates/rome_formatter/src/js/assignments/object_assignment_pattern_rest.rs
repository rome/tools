use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsObjectAssignmentPatternRest;
use rslint_syntax::JsObjectAssignmentPatternRestFields;

impl ToFormatElement for JsObjectAssignmentPatternRest {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternRestFields {
            dotdotdot_token,
            target,
        } = self.as_fields();

        Ok(format_elements![
            dotdotdot_token.format(formatter)?,
            target.format(formatter)?,
        ])
    }
}
