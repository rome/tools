use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsAnyType, TsAnyTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsAnyType;

impl FormatNodeRule<TsAnyType> for FormatTsAnyType {
    fn fmt_fields(&self, node: &TsAnyType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAnyTypeFields { any_token } = node.as_fields();

        write![f, [any_token.format()]]
    }
}
