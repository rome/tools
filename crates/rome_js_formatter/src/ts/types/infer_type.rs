use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsInferType, TsInferTypeFields};

impl FormatNodeFields<TsInferType> for FormatNodeRule<TsInferType> {
    fn format_fields(node: &TsInferType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsInferTypeFields {
            infer_token,
            type_parameter,
        } = node.as_fields();
        let infer = infer_token.format();
        let type_parameter = type_parameter.format();
        formatted![formatter, [infer, space_token(), type_parameter]]
    }
}
