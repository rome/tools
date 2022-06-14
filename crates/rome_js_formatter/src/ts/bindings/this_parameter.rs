use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsThisParameter, TsThisParameterFields};

impl FormatNodeFields<TsThisParameter> for FormatNodeRule<TsThisParameter> {
    fn fmt_fields(node: &TsThisParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let TsThisParameterFields {
            this_token,
            type_annotation,
        } = node.as_fields();

        write![f, [this_token.format(), type_annotation.format()]]
    }
}
