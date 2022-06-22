use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsThisType, TsThisTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsThisType;

impl FormatNodeRule<TsThisType> for FormatTsThisType {
    fn fmt_fields(&self, node: &TsThisType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsThisTypeFields { this_token } = node.as_fields();

        write![f, [this_token.format()]]
    }
}
