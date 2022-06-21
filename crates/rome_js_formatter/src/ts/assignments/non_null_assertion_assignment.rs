use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsNonNullAssertionAssignment;
use rome_js_syntax::TsNonNullAssertionAssignmentFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsNonNullAssertionAssignment;

impl FormatNodeRule<TsNonNullAssertionAssignment> for FormatTsNonNullAssertionAssignment {
    fn fmt_fields(
        &self,
        node: &TsNonNullAssertionAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsNonNullAssertionAssignmentFields {
            assignment,
            excl_token,
        } = node.as_fields();
        write![f, [assignment.format(), excl_token.format()]]
    }
}
