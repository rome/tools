use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsInferType, TsInferTypeFields};

impl FormatNodeFields<TsInferType> for FormatNodeRule<TsInferType> {
    fn fmt_fields(node: &TsInferType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsInferTypeFields {
            infer_token,
            type_parameter,
        } = node.as_fields();
        write![
            f,
            [infer_token.format(), space_token(), type_parameter.format()]
        ]
    }
}
