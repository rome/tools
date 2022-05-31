use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsThisParameter, TsThisParameterFields};

impl FormatNodeFields<TsThisParameter> for FormatNodeRule<TsThisParameter> {
    fn format_fields(
        node: &TsThisParameter,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsThisParameterFields {
            this_token,
            type_annotation,
        } = node.as_fields();

        formatted![formatter, [this_token.format(), type_annotation.format()]]
    }
}
