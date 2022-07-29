use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsInferType, TsInferTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsInferType;

impl FormatNodeRule<TsInferType> for FormatTsInferType {
    fn fmt_fields(&self, node: &TsInferType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsInferTypeFields {
            infer_token,
            type_parameter,
        } = node.as_fields();
        write![f, [infer_token.format(), space(), type_parameter.format()]]
    }
}
