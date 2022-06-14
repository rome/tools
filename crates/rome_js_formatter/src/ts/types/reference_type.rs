use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsReferenceType, TsReferenceTypeFields};

impl FormatNodeFields<TsReferenceType> for FormatNodeRule<TsReferenceType> {
    fn fmt_fields(node: &TsReferenceType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsReferenceTypeFields {
            name,
            type_arguments,
        } = node.as_fields();

        write![f, [name.format(), type_arguments.format()]]
    }
}
