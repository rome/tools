use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsNumberType, TsNumberTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNumberType;

impl FormatNodeRule<TsNumberType> for FormatTsNumberType {
    fn fmt_fields(&self, node: &TsNumberType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNumberTypeFields { number_token } = node.as_fields();

        write![f, [number_token.format()]]
    }
}
