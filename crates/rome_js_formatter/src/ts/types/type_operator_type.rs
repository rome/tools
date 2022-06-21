use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsTypeOperatorType, TsTypeOperatorTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeOperatorType;

impl FormatNodeRule<TsTypeOperatorType> for FormatTsTypeOperatorType {
    fn fmt_fields(node: &TsTypeOperatorType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeOperatorTypeFields { operator_token, ty } = node.as_fields();

        write![f, [operator_token.format(), space_token(), ty.format()]]
    }
}
