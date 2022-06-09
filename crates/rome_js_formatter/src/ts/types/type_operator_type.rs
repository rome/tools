use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsTypeOperatorType, TsTypeOperatorTypeFields};

impl FormatNodeFields<TsTypeOperatorType> for FormatNodeRule<TsTypeOperatorType> {
    fn fmt_fields(node: &TsTypeOperatorType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeOperatorTypeFields { operator_token, ty } = node.as_fields();

        write![f, [operator_token.format(), space_token(), ty.format()]]
    }
}
