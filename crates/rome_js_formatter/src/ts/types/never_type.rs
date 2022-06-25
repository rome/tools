use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsNeverType, TsNeverTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNeverType;

impl FormatNodeRule<TsNeverType> for FormatTsNeverType {
    fn fmt_fields(&self, node: &TsNeverType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNeverTypeFields { never_token } = node.as_fields();
        write![f, [never_token.format()]]
    }
}
