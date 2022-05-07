use crate::prelude::*;
use rome_js_syntax::TsNonNullAssertionAssignment;
use rome_js_syntax::TsNonNullAssertionAssignmentFields;

impl FormatNode for TsNonNullAssertionAssignment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsNonNullAssertionAssignmentFields {
            assignment,
            excl_token,
        } = self.as_fields();
        formatted![
            formatter,
            assignment.format(formatter)?,
            excl_token.format(formatter)?
        ]
    }
}
