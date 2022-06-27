use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsVoidType, TsVoidTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsVoidType;

impl FormatNodeRule<TsVoidType> for FormatTsVoidType {
    fn fmt_fields(&self, node: &TsVoidType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsVoidTypeFields { void_token } = node.as_fields();

        write![f, [void_token.format()]]
    }
}
