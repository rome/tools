use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsReferenceType, TsReferenceTypeFields};

impl FormatNodeFields<TsReferenceType> for FormatNodeRule<TsReferenceType> {
    fn format_fields(
        node: &TsReferenceType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsReferenceTypeFields {
            name,
            type_arguments,
        } = node.as_fields();

        formatted![formatter, [name.format(), type_arguments.format()]]
    }
}
