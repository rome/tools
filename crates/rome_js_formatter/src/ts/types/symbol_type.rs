use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsSymbolType, TsSymbolTypeFields};

impl FormatNodeFields<TsSymbolType> for FormatNodeRule<TsSymbolType> {
    fn fmt_fields(node: &TsSymbolType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsSymbolTypeFields { symbol_token } = node.as_fields();

        write![f, [symbol_token.format()]]
    }
}
